//! Cartridge OS Microkernel
//!
//! A minimal kernel (<1MB) providing:
//! - Memory management and isolation
//! - Capability-based access control
//! - IPC and shared memory
//! - Deterministic scheduling

#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)]

mod memory;
mod capability;
mod ipc;
mod scheduler;
mod syscall;
mod serial;
mod interrupts;

use core::panic::PanicInfo;
use core::alloc::{GlobalAlloc, Layout};

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // TODO: Zero BSS section (needs linker script fix)
    // For now, Rust initializes statics to zero anyway

    // Initialize serial port FIRST (our only output mechanism)
    serial::init();

    serial_println!("[KERNEL] Cartridge OS Kernel starting...");

    // Setup IDT before anything can fault
    interrupts::init();

    // Initialize kernel subsystems
    memory::init();
    capability::init();
    ipc::init();
    scheduler::init();
    syscall::init();

    serial_println!("[KERNEL] All subsystems initialized");

    // Show memory stats
    let stats = memory::get_stats();
    serial_println!("[KERNEL] Memory: {} bytes allocated, {} pages",
        stats.total_allocated, stats.page_count);

    // Hand off to switcher (loaded from initramfs)
    // TODO: Load and verify switcher artifact

    loop {
        // Halt CPU until next interrupt
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("\n[KERNEL PANIC]");

    if let Some(location) = info.location() {
        serial_println!("  Location: {}:{}:{}",
            location.file(), location.line(), location.column());
    }

    serial_println!("  Info: {}", info);
    serial_println!("System halted.");

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Allocation error handler
#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    serial_println!("[KERNEL] ALLOCATION FAILED: size={}, align={}",
        layout.size(), layout.align());
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Kernel logging function (routes to serial)
pub fn kernel_log(msg: &str) {
    serial_println!("{}", msg);
}

/// Kernel global allocator backed by physical memory allocator
#[global_allocator]
static ALLOCATOR: KernelAllocator = KernelAllocator;

struct KernelAllocator;

unsafe impl GlobalAlloc for KernelAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Use our physical memory allocator
        match memory::allocate_kernel(layout.size()) {
            Ok(ptr) => ptr,
            Err(_) => core::ptr::null_mut(),
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator doesn't support deallocation
        // This is acceptable for kernel use (allocations are permanent)
    }
}
