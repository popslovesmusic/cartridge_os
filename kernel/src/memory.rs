//! Memory management subsystem
//!
//! Provides physical memory allocation, page table setup, and memory isolation.
//!
//! Design:
//! - Deterministic bump allocator for boot-time allocations
//! - Page frame allocator (4KB pages) for runtime
//! - Isolated regions for kernel, drivers, and cartridges
//! - Zero-copy shared memory windows for high-throughput IPC

use core::sync::atomic::{AtomicUsize, Ordering};

/// Page size (4KB standard)
pub const PAGE_SIZE: usize = 4096;

/// Maximum physical memory supported (4GB for now)
pub const MAX_PHYSICAL_MEMORY: usize = 4 * 1024 * 1024 * 1024;

/// Kernel heap start (after kernel code/data)
const KERNEL_HEAP_START: usize = 0x10_0000; // 1MB mark

/// Physical memory allocator state
static ALLOCATOR: PhysicalAllocator = PhysicalAllocator::new();

/// Simple physical memory allocator
struct PhysicalAllocator {
    /// Next free physical address (bump allocator)
    next_free: AtomicUsize,
}

impl PhysicalAllocator {
    const fn new() -> Self {
        Self {
            next_free: AtomicUsize::new(KERNEL_HEAP_START),
        }
    }

    /// Allocate physically contiguous pages
    fn allocate_pages(&self, page_count: usize) -> Result<*mut u8, MemoryError> {
        let size = page_count * PAGE_SIZE;

        // Align to page boundary
        let current = self.next_free.load(Ordering::Acquire);
        let aligned = align_up(current, PAGE_SIZE);
        let new_addr = aligned + size;

        // Check bounds
        if new_addr > MAX_PHYSICAL_MEMORY {
            return Err(MemoryError::OutOfMemory);
        }

        // Atomic bump allocation
        match self.next_free.compare_exchange(
            current,
            new_addr,
            Ordering::Release,
            Ordering::Acquire,
        ) {
            Ok(_) => Ok(aligned as *mut u8),
            Err(_) => Err(MemoryError::AllocationRace),
        }
    }

    /// Get current allocation watermark (for diagnostics)
    fn current_usage(&self) -> usize {
        self.next_free.load(Ordering::Acquire) - KERNEL_HEAP_START
    }
}

/// Initialize memory subsystem
pub fn init() {
    // Memory map discovery happens in bootloader
    // Here we just initialize allocator state

    // TODO: Parse memory map from bootloader
    // TODO: Set up page tables (identity mapping + higher half)
    // TODO: Enable paging

    crate::serial_println!("[KERNEL] Memory subsystem initialized");
}

/// Allocate isolated memory region for a cartridge
pub fn allocate_cartridge_region(size: usize) -> Result<*mut u8, MemoryError> {
    // Round up to page boundary
    let pages = (size + PAGE_SIZE - 1) / PAGE_SIZE;

    ALLOCATOR.allocate_pages(pages)
}

/// Allocate kernel memory (for internal structures)
pub fn allocate_kernel(size: usize) -> Result<*mut u8, MemoryError> {
    let pages = (size + PAGE_SIZE - 1) / PAGE_SIZE;

    ALLOCATOR.allocate_pages(pages)
}

/// Map shared memory window for zero-copy IPC
///
/// This creates a shared mapping where both source and dest address spaces
/// can access the same physical pages. Critical for GPU/audio throughput.
pub fn map_shared_window(
    source: *mut u8,
    dest: *mut u8,
    size: usize,
) -> Result<(), MemoryError> {
    // Validate alignment
    if (source as usize) % PAGE_SIZE != 0 || (dest as usize) % PAGE_SIZE != 0 {
        return Err(MemoryError::InvalidAddress);
    }

    // TODO: Implement page table manipulation
    // 1. Get physical address of source pages
    // 2. Map same physical pages into dest virtual address space
    // 3. Set appropriate permissions (read/write, user accessible)

    Ok(())
}

/// Get memory usage statistics
pub fn get_stats() -> MemoryStats {
    MemoryStats {
        total_allocated: ALLOCATOR.current_usage(),
        page_count: ALLOCATOR.current_usage() / PAGE_SIZE,
    }
}

/// Memory usage statistics
#[derive(Debug, Clone, Copy)]
pub struct MemoryStats {
    pub total_allocated: usize,
    pub page_count: usize,
}

/// Align address up to boundary
#[inline]
const fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

/// Memory error types
#[derive(Debug)]
pub enum MemoryError {
    OutOfMemory,
    InvalidAddress,
    AlreadyMapped,
    AllocationRace,
}
