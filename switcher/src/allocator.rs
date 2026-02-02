//! Simple bump allocator for switcher
//!
//! Uses kernel memory allocation syscall as backing store

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

use crate::syscall::sys_allocate_memory;

/// Global allocator for switcher (bump allocator using kernel memory)
pub struct SwitcherAllocator {
    /// Next free address within current chunk
    next_free: AtomicUsize,
    /// End of current chunk
    chunk_end: AtomicUsize,
}

impl SwitcherAllocator {
    pub const fn new() -> Self {
        Self {
            next_free: AtomicUsize::new(0),
            chunk_end: AtomicUsize::new(0),
        }
    }

    /// Allocate a new chunk from kernel
    fn allocate_chunk(&self, size: usize) -> *mut u8 {
        let result = sys_allocate_memory(size);

        if result.is_ok() {
            result.value as *mut u8
        } else {
            null_mut()
        }
    }
}

unsafe impl GlobalAlloc for SwitcherAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // Get current allocation pointer
        let mut current = self.next_free.load(Ordering::Relaxed);

        loop {
            // If no chunk allocated yet, or current chunk exhausted
            if current == 0 || current >= self.chunk_end.load(Ordering::Relaxed) {
                // Allocate new chunk (at least 4KB, or requested size)
                let chunk_size = size.max(4096);
                let chunk_ptr = self.allocate_chunk(chunk_size);

                if chunk_ptr.is_null() {
                    return null_mut();
                }

                self.next_free.store(chunk_ptr as usize, Ordering::Relaxed);
                self.chunk_end.store(chunk_ptr as usize + chunk_size, Ordering::Relaxed);
                current = chunk_ptr as usize;
            }

            // Align up to requested alignment
            let aligned = align_up(current, align);

            // Check if allocation fits in current chunk
            let end = aligned + size;
            if end > self.chunk_end.load(Ordering::Relaxed) {
                // Doesn't fit, allocate new chunk
                current = 0;
                continue;
            }

            // Try to reserve this range
            match self.next_free.compare_exchange(
                current,
                end,
                Ordering::Relaxed,
                Ordering::Relaxed,
            ) {
                Ok(_) => return aligned as *mut u8,
                Err(updated) => current = updated,
            }
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator doesn't support deallocation
        // Memory is reclaimed when switcher exits
    }
}

/// Align address up to alignment boundary
#[inline]
const fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

/// Global allocator instance
#[global_allocator]
pub static ALLOCATOR: SwitcherAllocator = SwitcherAllocator::new();

/// Allocation error handler
#[alloc_error_handler]
fn alloc_error(layout: Layout) -> ! {
    panic!("Allocation failed: size={}, align={}", layout.size(), layout.align());
}
