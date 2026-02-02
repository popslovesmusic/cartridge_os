//! Cartridge Switcher
//!
//! Userspace component that discovers, verifies, and loads cartridges.
//! Acts as the first userspace program, launched by the kernel.
//!
//! Responsibilities:
//! - Discover available cartridges from storage
//! - Verify cartridge integrity (hash/signature)
//! - Load required driver modules
//! - Request kernel to allocate isolated memory for cartridge
//! - Transfer control to cartridge entry point
//!
//! Current implementation (Phase 2.4):
//! - Basic syscall integration
//! - Logs to kernel console
//! - Prepares for cartridge loading (Phase 2.5)

#![no_std]
#![no_main]
#![feature(alloc_error_handler)]

extern crate alloc;

mod allocator;
mod syscall;

use core::panic::PanicInfo;
use syscall::{sys_exit, sys_get_memory_stats};

/// Switcher entry point
///
/// Called by kernel after boot. Runs in userspace with limited privileges.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    switcher_log!("[SWITCHER] Cartridge OS Switcher v0.1.0");
    switcher_log!("[SWITCHER] Initializing userspace environment...");

    // Show memory stats from kernel
    let mem_stats = sys_get_memory_stats();
    if mem_stats.is_ok() {
        switcher_log!("[SWITCHER] Kernel memory allocated: {} bytes", mem_stats.value as u64);
    }

    switcher_log!("[SWITCHER] ✓ Switcher initialized");
    switcher_log!("");

    // Phase 2.4: Basic functionality
    // Phase 2.5: Will discover and load actual cartridges
    switcher_log!("[SWITCHER] Cartridge discovery not yet implemented");
    switcher_log!("[SWITCHER] Waiting for Phase 2.5 (Hello World cartridge)");
    switcher_log!("");

    // For now, enter idle loop
    // Future: discover cartridges, present menu, load selected cartridge
    idle_loop()
}

/// Idle loop - waits for events
///
/// Future: Will handle:
/// - User input for cartridge selection
/// - Hotplug events
/// - Cartridge crash recovery
fn idle_loop() -> ! {
    switcher_log!("[SWITCHER] Entering idle mode");
    switcher_log!("[SWITCHER] System ready");

    loop {
        // Halt CPU until interrupt
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}

/// Panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    switcher_log!("");
    switcher_log!("[SWITCHER PANIC]");

    if let Some(location) = info.location() {
        switcher_log!("  Location: {}:{}:{}",
            location.file(), location.line(), location.column());
    }

    switcher_log!("  Info: {}", info);

    switcher_log!("[SWITCHER] Exiting with error code 1");

    sys_exit(1);
}

// Future functions for Phase 2.5+

/// Discover cartridges from storage partition
#[allow(dead_code)]
fn discover_cartridges() -> alloc::vec::Vec<&'static str> {
    // TODO: Scan cartridge partition for .cart files
    // TODO: Parse manifest to get cartridge metadata
    alloc::vec::Vec::new()
}

/// Verify and load a cartridge artifact
#[allow(dead_code)]
fn load_cartridge(_path: &str) {
    // TODO: Read cartridge file from storage
    // TODO: Verify artifact integrity (Artifact::from_bytes)
    // TODO: Check capabilities
    // TODO: Load required drivers
    // TODO: Allocate isolated memory via syscall
    // TODO: Transfer control to cartridge
}
