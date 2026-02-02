//! Hello World Cartridge
//!
//! The first Cartridge OS application - demonstrates:
//! - Verified boot chain (firmware → bootloader → kernel → switcher → cartridge)
//! - Tanka metadata validation
//! - Syscall interface for logging
//! - Minimal no_std binary
//!
//! Tanka (5-7-5-7-7):
//!   First light awakens
//!   Console whispers soft greeting
//!   Trust verified
//!   Minimal code executes clean
//!   Cartridge system proven sound

#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Syscall numbers (must match kernel/switcher)
#[repr(u64)]
enum Syscall {
    Log = 0,
    Exit = 5,
}

/// Cartridge entry point
///
/// Called by switcher after verification and memory allocation
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // Log hello message to kernel console
    log("Hello, Cartridge OS!");
    log("This is the first verified cartridge");
    log("");
    log("Boot chain complete:");
    log("  ✓ Firmware verified bootloader");
    log("  ✓ Bootloader verified kernel (SHA-256)");
    log("  ✓ Kernel started switcher");
    log("  ✓ Switcher verified this cartridge");
    log("  ✓ Tanka validated (5-7-5-7-7)");
    log("");
    log("Cartridge execution successful!");

    // Exit cleanly
    exit(0);
}

/// Log message via syscall
fn log(msg: &str) {
    let ptr = msg.as_ptr() as u64;
    let len = msg.len() as u64;

    unsafe {
        syscall2(Syscall::Log as u64, ptr, len);
    }
}

/// Exit cartridge
fn exit(code: u64) -> ! {
    unsafe {
        syscall1(Syscall::Exit as u64, code);
    }
    unreachable!()
}

/// Low-level syscall with 1 argument
#[inline]
unsafe fn syscall1(num: u64, arg1: u64) -> u64 {
    let result: u64;
    core::arch::asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        lateout("rax") result,
        options(nostack)
    );
    result
}

/// Low-level syscall with 2 arguments
#[inline]
unsafe fn syscall2(num: u64, arg1: u64, arg2: u64) -> u64 {
    let result: u64;
    core::arch::asm!(
        "syscall",
        in("rax") num,
        in("rdi") arg1,
        in("rsi") arg2,
        lateout("rax") result,
        options(nostack)
    );
    result
}

/// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    log("[PANIC] Cartridge panicked");
    exit(1);
}
