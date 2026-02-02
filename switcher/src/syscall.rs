//! Syscall interface for switcher
//!
//! Provides userspace wrappers around kernel syscalls

use core::arch::asm;

/// Syscall numbers (must match kernel/src/syscall.rs)
#[repr(u64)]
#[derive(Debug, Clone, Copy)]
pub enum SyscallNumber {
    Log = 0,
    AllocateMemory = 1,
    CreateChannel = 2,
    SendMessage = 3,
    RecvMessage = 4,
    Exit = 5,
    GetMemoryStats = 6,
    MapSharedWindow = 7,
}

/// Syscall result
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SyscallResult {
    pub value: i64,
    pub value2: u64,
}

impl SyscallResult {
    pub fn is_ok(&self) -> bool {
        self.value >= 0
    }

    pub fn is_err(&self) -> bool {
        self.value < 0
    }

    pub fn unwrap(&self) -> u64 {
        if self.is_ok() {
            self.value as u64
        } else {
            panic!("Syscall failed with error code: {}", -self.value);
        }
    }

    pub fn ok(&self) -> Option<u64> {
        if self.is_ok() {
            Some(self.value as u64)
        } else {
            None
        }
    }
}

/// Log message to kernel console
///
/// # Safety
/// Caller must ensure the string is valid UTF-8 and the pointer is valid
pub unsafe fn sys_log(msg: &str) -> SyscallResult {
    let ptr = msg.as_ptr() as u64;
    let len = msg.len() as u64;

    syscall2(SyscallNumber::Log as u64, ptr, len)
}

/// Allocate memory region
///
/// Returns base address on success
pub fn sys_allocate_memory(size: usize) -> SyscallResult {
    syscall1(SyscallNumber::AllocateMemory as u64, size as u64)
}

/// Exit current process
pub fn sys_exit(code: u64) -> ! {
    syscall1(SyscallNumber::Exit as u64, code);
    unreachable!()
}

/// Get memory statistics
pub fn sys_get_memory_stats() -> SyscallResult {
    syscall0(SyscallNumber::GetMemoryStats as u64)
}

// Low-level syscall wrappers using x86_64 syscall instruction

#[inline]
fn syscall0(num: u64) -> SyscallResult {
    let value: i64;
    let value2: u64;

    unsafe {
        asm!(
            "syscall",
            in("rax") num,
            lateout("rax") value,
            lateout("rdx") value2,
            options(nostack)
        );
    }

    SyscallResult { value, value2 }
}

#[inline]
fn syscall1(num: u64, arg1: u64) -> SyscallResult {
    let value: i64;
    let value2: u64;

    unsafe {
        asm!(
            "syscall",
            in("rax") num,
            in("rdi") arg1,
            lateout("rax") value,
            lateout("rdx") value2,
            options(nostack)
        );
    }

    SyscallResult { value, value2 }
}

#[inline]
fn syscall2(num: u64, arg1: u64, arg2: u64) -> SyscallResult {
    let value: i64;
    let value2: u64;

    unsafe {
        asm!(
            "syscall",
            in("rax") num,
            in("rdi") arg1,
            in("rsi") arg2,
            lateout("rax") value,
            lateout("rdx") value2,
            options(nostack)
        );
    }

    SyscallResult { value, value2 }
}

#[inline]
#[allow(dead_code)]
fn syscall3(num: u64, arg1: u64, arg2: u64, arg3: u64) -> SyscallResult {
    let value: i64;
    let value2: u64;

    unsafe {
        asm!(
            "syscall",
            in("rax") num,
            in("rdi") arg1,
            in("rsi") arg2,
            in("rdx") arg3,
            lateout("rax") value,
            lateout("rdx") value2,
            options(nostack)
        );
    }

    SyscallResult { value, value2 }
}

/// Helper macro for logging from switcher
#[macro_export]
macro_rules! switcher_log {
    ($($arg:tt)*) => {{
        use core::fmt::Write;
        let mut buf = [0u8; 256];
        let mut writer = $crate::syscall::StrWriter::new(&mut buf);
        let _ = write!(&mut writer, $($arg)*);
        let msg = writer.as_str();
        unsafe { $crate::syscall::sys_log(msg) };
    }};
}

/// Simple string writer for formatting
pub struct StrWriter<'a> {
    buf: &'a mut [u8],
    pos: usize,
}

impl<'a> StrWriter<'a> {
    pub fn new(buf: &'a mut [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8(&self.buf[..self.pos]).unwrap_or("")
    }
}

impl<'a> core::fmt::Write for StrWriter<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let bytes = s.as_bytes();
        let available = self.buf.len() - self.pos;
        let to_write = bytes.len().min(available);

        self.buf[self.pos..self.pos + to_write].copy_from_slice(&bytes[..to_write]);
        self.pos += to_write;

        Ok(())
    }
}
