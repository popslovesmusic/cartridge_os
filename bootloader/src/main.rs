//! UEFI Bootloader
//!
//! Verifies kernel hash and hands off control.
//! This is the first link in the chain of trust.
//!
//! Architect requirement: "Verify kernel hash before handoff"

#![no_std]
#![no_main]

extern crate alloc;

use alloc::vec::Vec;
use cartridge_common::{Artifact, ArtifactType};
use uefi::allocator::Allocator;
use uefi::prelude::*;
use uefi::proto::console::text::Output;
use uefi::proto::media::file::{File, FileAttribute, FileMode};
use uefi::proto::media::fs::SimpleFileSystem;
use uefi::table::boot::MemoryType;
use uefi::CString16;

/// Global allocator for bootloader (uses UEFI boot services)
#[global_allocator]
static ALLOCATOR: Allocator = Allocator;

/// Bootloader entry point (UEFI application)
#[entry]
fn main(image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    // Initialize UEFI helpers (logging, allocator, panic handler)
    uefi::helpers::init(&mut system_table).expect("Failed to initialize UEFI helpers");

    {
        let stdout = system_table.stdout();
        boot_log(stdout, "[BOOTLOADER] Cartridge OS Bootloader v0.1.0");
        boot_log(stdout, "[BOOTLOADER] Loading kernel from /kernel.bin");
    }

    // Step 1: Load kernel from ESP (EFI System Partition)
    let kernel_data = {
        let boot_services = system_table.boot_services();

        match load_kernel_from_esp(boot_services) {
            Ok(data) => {
                let stdout = system_table.stdout();
                boot_log(stdout, "[BOOTLOADER] Kernel loaded successfully");
                data
            }
            Err(_) => {
                let stdout = system_table.stdout();
                boot_log(stdout, "[BOOTLOADER] ERROR: Failed to load kernel");
                loop {}
            }
        }
    };

    // Step 2: Verify kernel artifact (hash verification)
    let artifact = {
        let stdout = system_table.stdout();
        boot_log(stdout, "[BOOTLOADER] Verifying kernel integrity...");

        match Artifact::from_bytes(&kernel_data) {
            Ok(art) => {
                boot_log(stdout, "[BOOTLOADER] ✓ Kernel hash verified");
                art
            }
            Err(_) => {
                boot_log(stdout, "[BOOTLOADER] ✗ KERNEL VERIFICATION FAILED");
                boot_log(stdout, "[BOOTLOADER] Hash mismatch - kernel may be corrupted");
                boot_log(stdout, "[BOOTLOADER] SYSTEM HALTED");
                loop {}
            }
        }
    };

    // Step 3: Validate artifact type
    {
        let stdout = system_table.stdout();
        if artifact.header.artifact_type != ArtifactType::Kernel as u8 {
            boot_log(stdout, "[BOOTLOADER] ✗ Invalid artifact type");
            loop {}
        }
        boot_log(stdout, "[BOOTLOADER] ✓ Kernel artifact validated");
    }

    // Step 4: Allocate memory for kernel at 1MB physical address
    let kernel_base = 0x100000u64;
    let kernel_pages = (artifact.payload.len() + 4095) / 4096;

    {
        let stdout = system_table.stdout();
        boot_log(stdout, "[BOOTLOADER] Allocating kernel memory...");
    }

    // Use LOADER_CODE for executable kernel code (not LOADER_DATA)
    let _kernel_mem = system_table.boot_services()
        .allocate_pages(
            uefi::table::boot::AllocateType::Address(kernel_base),
            MemoryType::LOADER_CODE,
            kernel_pages,
        )
        .expect("Failed to allocate kernel memory");

    // Allocate 64KB stack for kernel (16 pages of 4KB each)
    // Stack grows downward, so we allocate at a higher address
    let stack_top = 0x80000u64;  // Stack at 512KB
    let stack_pages: usize = 16;  // 64KB stack

    {
        let stdout = system_table.stdout();
        boot_log(stdout, "[BOOTLOADER] Allocating kernel stack...");
    }

    let _stack_mem = system_table.boot_services()
        .allocate_pages(
            uefi::table::boot::AllocateType::Address(stack_top - (stack_pages as u64 * 4096)),
            MemoryType::LOADER_DATA,  // Stack can be data memory
            stack_pages,
        )
        .expect("Failed to allocate kernel stack");

    // Step 5: Copy kernel to memory
    unsafe {
        core::ptr::copy_nonoverlapping(
            artifact.payload.as_ptr(),
            kernel_base as usize as *mut u8,
            artifact.payload.len(),
        );
    }

    {
        let stdout = system_table.stdout();
        boot_log(stdout, "[BOOTLOADER] ✓ Kernel loaded at 0x100000");
        boot_log(stdout, "[BOOTLOADER] Exiting boot services...");
        boot_log(stdout, "[BOOTLOADER] Transferring control to kernel...");
        boot_log(stdout, "");
    }

    // Exit boot services (SAFETY: final action before kernel handoff)
    let (_system_table, _memory_map) = unsafe {
        system_table.exit_boot_services(MemoryType::LOADER_DATA)
    };

    // Jump to kernel entry point with proper stack setup
    let kernel_entry_addr = kernel_base as usize + artifact.header.entry_offset as usize;

    unsafe {
        // Set up stack pointer and jump to kernel
        // The kernel expects RSP to be set to a valid stack
        core::arch::asm!(
            "mov rsp, {stack_top}",
            "jmp {entry}",
            stack_top = in(reg) stack_top,
            entry = in(reg) kernel_entry_addr,
            options(noreturn)
        );
    }
}

/// Load kernel from EFI System Partition
fn load_kernel_from_esp(
    boot_services: &BootServices,
) -> Result<Vec<u8>, ()> {
    // Get simple file system protocol
    let fs_handle = boot_services
        .get_handle_for_protocol::<SimpleFileSystem>()
        .map_err(|_| ())?;

    let mut fs = boot_services
        .open_protocol_exclusive::<SimpleFileSystem>(fs_handle)
        .map_err(|_| ())?;

    // Open root directory
    let mut root = fs.open_volume().map_err(|_| ())?;

    // Open kernel file
    let kernel_handle = root
        .open(
            cstr16!("kernel.bin"),
            FileMode::Read,
            FileAttribute::empty(),
        )
        .map_err(|_| ())?;

    // Convert to RegularFile for reading
    let mut kernel_file = kernel_handle.into_regular_file().ok_or(())?;

    // Get file size
    let mut info_buffer = [0u8; 256];
    let file_info = kernel_file
        .get_info::<uefi::proto::media::file::FileInfo>(&mut info_buffer)
        .map_err(|_| ())?;

    let file_size = file_info.file_size() as usize;

    // Read entire file
    let mut buffer = Vec::with_capacity(file_size);
    buffer.resize(file_size, 0);

    kernel_file.read(&mut buffer).map_err(|_| ())?;

    Ok(buffer)
}

/// Log message to UEFI console
fn boot_log(stdout: &mut Output, msg: &str) {
    // Convert Rust string to UCS-2 for UEFI
    if let Ok(ucs2_msg) = CString16::try_from(msg) {
        stdout.output_string(&ucs2_msg).ok();
    }
    stdout.output_string(cstr16!("\r\n")).ok();
}

/// Panic handler
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
