//! IPC subsystem
//!
//! Provides zero-copy shared memory windows for high-throughput communication
//! between cartridges and drivers (GPU, audio, etc.)
//!
//! Design:
//! - AVX2-aligned shared memory regions (32-byte alignment)
//! - Capability-gated access (checked at creation time)
//! - Lock-free ring buffers for async message passing
//! - Direct memory mapping (no copy between address spaces)

use core::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use crate::memory;

/// AVX2 cache line size (32 bytes)
pub const CACHE_LINE_SIZE: usize = 32;

/// Maximum number of concurrent IPC channels
const MAX_CHANNELS: usize = 256;

/// Shared memory window descriptor
#[repr(C, align(32))]  // AVX2 alignment
pub struct SharedWindow {
    /// Physical base address of shared region
    phys_addr: usize,

    /// Size in bytes (must be page-aligned)
    size: usize,

    /// Owner cartridge ID
    owner_id: u64,

    /// Peer cartridge/driver ID
    peer_id: u64,

    /// Read offset (for ring buffer)
    read_offset: AtomicUsize,

    /// Write offset (for ring buffer)
    write_offset: AtomicUsize,

    /// Window state
    state: AtomicU64,
}

/// IPC channel registry
static CHANNELS: [AtomicU64; MAX_CHANNELS] = {
    const EMPTY: AtomicU64 = AtomicU64::new(0);
    [EMPTY; MAX_CHANNELS]
};

/// Next available channel ID
static NEXT_CHANNEL_ID: AtomicU64 = AtomicU64::new(1);

/// Initialize IPC subsystem
pub fn init() {
    crate::serial_println!("[KERNEL] IPC subsystem initialized");
}

/// Create zero-copy shared memory window between two components
pub fn create_shared_window(
    owner: u64,
    peer: u64,
    size: usize,
) -> Result<ChannelId, IpcError> {
    // Validate size is page-aligned
    if size % memory::PAGE_SIZE != 0 {
        return Err(IpcError::InvalidSize);
    }

    // TODO: Check capability - does owner have IPC_SHARED_MEMORY?

    // Allocate shared memory region
    let phys_addr = memory::allocate_kernel(size)
        .map_err(|_| IpcError::OutOfMemory)? as usize;

    // Allocate channel ID
    let channel_id = NEXT_CHANNEL_ID.fetch_add(1, Ordering::Relaxed);

    // TODO: Store window descriptor in channel registry
    // TODO: Map shared region into both address spaces

    Ok(channel_id)
}

/// Map shared window into cartridge address space
pub fn map_window_to_cartridge(
    channel: ChannelId,
    cartridge_id: u64,
    virt_addr: usize,
) -> Result<(), IpcError> {
    // TODO: Validate channel ownership
    // TODO: Get physical address from channel descriptor
    // TODO: Call memory::map_shared_window()

    Ok(())
}

/// Write to shared window (zero-copy)
pub fn write_to_window(
    channel: ChannelId,
    data: &[u8],
) -> Result<usize, IpcError> {
    // TODO: Get window descriptor
    // TODO: Check write offset + data.len() doesn't overflow
    // TODO: Copy data directly into shared region
    // TODO: Update write_offset atomically

    Ok(data.len())
}

/// Read from shared window (zero-copy)
pub fn read_from_window(
    channel: ChannelId,
    buffer: &mut [u8],
) -> Result<usize, IpcError> {
    // TODO: Get window descriptor
    // TODO: Check available data (write_offset - read_offset)
    // TODO: Copy from shared region to buffer
    // TODO: Update read_offset atomically

    Ok(0)
}

/// Get direct pointer to shared window (for SIMD operations)
///
/// SAFETY: Caller must ensure:
/// - Access is within bounds
/// - Proper synchronization with peer
/// - AVX2 alignment is maintained
pub unsafe fn get_window_ptr(channel: ChannelId) -> Result<*mut u8, IpcError> {
    // TODO: Validate channel
    // TODO: Return physical address cast to pointer

    Err(IpcError::InvalidChannel)
}

/// Legacy message passing (uses shared window internally)
pub fn create_channel(source: u64, dest: u64) -> Result<ChannelId, IpcError> {
    // Default 4KB message buffer
    create_shared_window(source, dest, 4096)
}

pub fn send_message(channel: ChannelId, data: &[u8]) -> Result<(), IpcError> {
    write_to_window(channel, data)?;
    Ok(())
}

pub fn recv_message(channel: ChannelId, buffer: &mut [u8]) -> Result<usize, IpcError> {
    read_from_window(channel, buffer)
}

/// Channel identifier
pub type ChannelId = u64;

/// Window state flags
#[derive(Debug, Clone, Copy)]
#[repr(u64)]
pub enum WindowState {
    Inactive = 0,
    Active = 1,
    Closing = 2,
}

/// IPC error types
#[derive(Debug)]
pub enum IpcError {
    CapabilityDenied,
    InvalidChannel,
    BufferTooSmall,
    InvalidSize,
    OutOfMemory,
    WindowFull,
}
