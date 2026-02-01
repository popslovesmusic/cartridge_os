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

use core::panic::PanicInfo;
use core::alloc::{GlobalAlloc, Layout};

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize kernel subsystems
    memory::init();
    capability::init();
    ipc::init();
    scheduler::init();
    syscall::init();

    // Boot message
    kernel_log("Cartridge OS Kernel initialized");

    // Show memory stats
    let stats = memory::get_stats();
    kernel_log("Memory allocator ready");
    kernel_log("Syscall interface ready");

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
    kernel_log("KERNEL PANIC");

    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Allocation error handler
#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    kernel_log("ALLOCATION FAILED");
    loop {
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Simple kernel logging (will output to serial/screen)
pub fn kernel_log(_msg: &str) {
    // TODO: Implement proper logging to serial port
    // For now, this is a placeholder
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
