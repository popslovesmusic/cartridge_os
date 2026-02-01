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

mod memory;
mod capability;
mod ipc;
mod scheduler;

use core::panic::PanicInfo;

/// Kernel entry point
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Initialize kernel subsystems
    memory::init();
    capability::init();
    ipc::init();
    scheduler::init();

    // Boot message
    kernel_log("Cartridge OS Kernel initialized");

    // Hand off to switcher (loaded from initramfs)
    // TODO: Load and verify switcher artifact

    loop {
        // Halt CPU until next interrupt
        x86_64::instructions::hlt();
    }
}

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel_log("KERNEL PANIC");
    if let Some(location) = info.location() {
        kernel_log(&alloc::format!(
            "at {}:{}:{}",
            location.file(),
            location.line(),
            location.column()
        ));
    }

    loop {
        x86_64::instructions::hlt();
    }
}

/// Simple kernel logging (will output to serial/screen)
fn kernel_log(msg: &str) {
    // TODO: Implement proper logging to serial port
    // For now, this is a placeholder
}

// Allocator stub (will implement proper allocator)
#[global_allocator]
static ALLOCATOR: DummyAllocator = DummyAllocator;

struct DummyAllocator;

unsafe impl core::alloc::GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: core::alloc::Layout) -> *mut u8 {
        core::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: core::alloc::Layout) {
        // No-op
    }
}

extern crate alloc;
