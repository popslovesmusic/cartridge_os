//! Syscall interface
//!
//! Provides system call interface for userspace (switcher, cartridges) to
//! interact with the kernel.
//!
//! Design:
//! - x86_64 syscall instruction (fast path)
//! - Capability checks on every syscall
//! - Minimal syscall surface (only essential operations)

use crate::memory;
use crate::ipc;

/// Syscall numbers
#[repr(u64)]
#[derive(Debug, Clone, Copy)]
pub enum SyscallNumber {
    /// Log message to kernel console
    Log = 0,

    /// Allocate memory region
    AllocateMemory = 1,

    /// Create IPC channel
    CreateChannel = 2,

    /// Send IPC message
    SendMessage = 3,

    /// Receive IPC message
    RecvMessage = 4,

    /// Exit current cartridge
    Exit = 5,

    /// Get memory statistics
    GetMemoryStats = 6,

    /// Map shared memory window
    MapSharedWindow = 7,
}

/// Syscall arguments (passed in registers)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SyscallArgs {
    pub syscall_num: u64,
    pub arg1: u64,
    pub arg2: u64,
    pub arg3: u64,
    pub arg4: u64,
    pub arg5: u64,
}

/// Syscall result
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct SyscallResult {
    /// Return value (0 = success, negative = error code)
    pub value: i64,

    /// Optional secondary return value
    pub value2: u64,
}

impl SyscallResult {
    pub const fn ok(value: u64) -> Self {
        Self {
            value: value as i64,
            value2: 0,
        }
    }

    pub const fn ok2(value: u64, value2: u64) -> Self {
        Self {
            value: value as i64,
            value2,
        }
    }

    pub const fn err(code: SyscallError) -> Self {
        Self {
            value: -(code as i64),
            value2: 0,
        }
    }
}

/// Syscall error codes
#[repr(i64)]
#[derive(Debug, Clone, Copy)]
pub enum SyscallError {
    InvalidSyscall = 1,
    PermissionDenied = 2,
    InvalidArgument = 3,
    OutOfMemory = 4,
    NotFound = 5,
    AlreadyExists = 6,
}

/// Initialize syscall subsystem
pub fn init() {
    // TODO: Set up syscall entry point in MSR
    // MSR_LSTAR = syscall_entry
    // MSR_STAR = kernel/user CS/SS
    // MSR_SFMASK = RFLAGS mask

    crate::serial_println!("[KERNEL] Syscall subsystem initialized");
}

/// Main syscall dispatcher
///
/// Called from assembly syscall entry point with arguments in registers
pub fn dispatch(args: SyscallArgs) -> SyscallResult {
    // TODO: Get current cartridge ID from CPU-local storage

    match args.syscall_num {
        0 => syscall_log(args),
        1 => syscall_allocate_memory(args),
        2 => syscall_create_channel(args),
        3 => syscall_send_message(args),
        4 => syscall_recv_message(args),
        5 => syscall_exit(args),
        6 => syscall_get_memory_stats(args),
        7 => syscall_map_shared_window(args),
        _ => SyscallResult::err(SyscallError::InvalidSyscall),
    }
}

/// Syscall: Log message
///
/// arg1 = pointer to string
/// arg2 = length
fn syscall_log(args: SyscallArgs) -> SyscallResult {
    let ptr = args.arg1 as *const u8;
    let len = args.arg2 as usize;

    // TODO: Validate pointer is in userspace
    // TODO: Read string from userspace memory

    crate::kernel_log("[USERSPACE LOG]");

    SyscallResult::ok(0)
}

/// Syscall: Allocate memory
///
/// arg1 = size in bytes
/// Returns: base address
fn syscall_allocate_memory(args: SyscallArgs) -> SyscallResult {
    let size = args.arg1 as usize;

    // TODO: Check capability - does cartridge have STORAGE or similar?

    match memory::allocate_cartridge_region(size) {
        Ok(ptr) => SyscallResult::ok(ptr as u64),
        Err(_) => SyscallResult::err(SyscallError::OutOfMemory),
    }
}

/// Syscall: Create IPC channel
///
/// arg1 = peer cartridge/driver ID
/// arg2 = size
/// Returns: channel ID
fn syscall_create_channel(args: SyscallArgs) -> SyscallResult {
    let peer_id = args.arg1;
    let size = args.arg2 as usize;

    // TODO: Get current cartridge ID
    let owner_id = 0; // Placeholder

    // TODO: Check capability - IPC_BASIC or IPC_SHARED_MEMORY

    match ipc::create_shared_window(owner_id, peer_id, size) {
        Ok(channel_id) => SyscallResult::ok(channel_id),
        Err(_) => SyscallResult::err(SyscallError::PermissionDenied),
    }
}

/// Syscall: Send message over IPC
///
/// arg1 = channel ID
/// arg2 = pointer to data
/// arg3 = length
fn syscall_send_message(args: SyscallArgs) -> SyscallResult {
    let channel_id = args.arg1;
    let _ptr = args.arg2 as *const u8;
    let _len = args.arg3 as usize;

    // TODO: Validate pointer
    // TODO: Read data from userspace
    // TODO: Call ipc::write_to_window()

    SyscallResult::ok(0)
}

/// Syscall: Receive message from IPC
///
/// arg1 = channel ID
/// arg2 = pointer to buffer
/// arg3 = buffer length
/// Returns: bytes received
fn syscall_recv_message(args: SyscallArgs) -> SyscallResult {
    let _channel_id = args.arg1;
    let _ptr = args.arg2 as *mut u8;
    let _len = args.arg3 as usize;

    // TODO: Validate pointer
    // TODO: Call ipc::read_from_window()
    // TODO: Copy to userspace buffer

    SyscallResult::ok(0)
}

/// Syscall: Exit cartridge
///
/// arg1 = exit code
fn syscall_exit(args: SyscallArgs) -> SyscallResult {
    let _exit_code = args.arg1;

    // TODO: Terminate current cartridge
    // TODO: Clean up resources
    // TODO: Return to switcher

    crate::kernel_log("Cartridge exit requested");

    // This syscall doesn't return
    SyscallResult::ok(0)
}

/// Syscall: Get memory statistics
///
/// Returns: total allocated bytes
fn syscall_get_memory_stats(_args: SyscallArgs) -> SyscallResult {
    let stats = memory::get_stats();
    SyscallResult::ok(stats.total_allocated as u64)
}

/// Syscall: Map shared window
///
/// arg1 = channel ID
/// arg2 = virtual address to map at
/// Returns: physical address
fn syscall_map_shared_window(args: SyscallArgs) -> SyscallResult {
    let channel_id = args.arg1;
    let virt_addr = args.arg2 as usize;

    // TODO: Get current cartridge ID
    let cartridge_id = 0;

    // TODO: Validate virtual address is in userspace
    // TODO: Call ipc::map_window_to_cartridge()

    match ipc::map_window_to_cartridge(channel_id, cartridge_id, virt_addr) {
        Ok(_) => SyscallResult::ok(virt_addr as u64),
        Err(_) => SyscallResult::err(SyscallError::InvalidArgument),
    }
}
