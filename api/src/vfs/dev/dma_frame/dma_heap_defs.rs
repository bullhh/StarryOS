//! DMA heap data structures and constants

use bytemuck::{Pod, Zeroable};

/// DMA heap allocation data structure (Linux compatible)
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct DmaHeapAllocationData {
    /// Size of the buffer to allocate
    pub len: u64,
    /// File descriptor to the dma-buf
    pub fd: i32,
    /// Flags for the allocation
    pub fd_flags: u32,
}

impl DmaHeapAllocationData {
    /// Creates a new allocation data structure
    pub fn new(len: u64, fd_flags: u32) -> Self {
        Self {
            len,
            fd: -1, // Initialize with invalid fd
            fd_flags,
        }
    }
}

/// DMA buffer sync directions
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DmaBufSyncDirection {
    /// Read from the buffer
    ToDevice = 0,
    /// Write to the buffer
    FromDevice = 1,
    /// Read and write to the buffer
    Both = 2,
}

/// DMA buffer sync flags
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct DmaBufSync {
    /// Start of the range to sync
    pub start: u64,
    /// End of the range to sync
    pub end: u64,
    /// Sync direction and flags
    pub flags: u32,
    /// Padding to ensure proper alignment
    _pad: u32,
}

impl DmaBufSync {
    /// Creates a new sync structure
    pub fn new(start: u64, end: u64, direction: DmaBufSyncDirection) -> Self {
        Self {
            start,
            end,
            flags: direction as u32,
            _pad: 0,
        }
    }
}

// Common ioctl commands
/// Allocate a dma-buf
pub const DMA_HEAP_IOCTL_ALLOC: u32 = 0xc0104800;
/// Sync a dma-buf
pub const DMA_BUF_IOCTL_SYNC: u32 = 0xc0086200;
/// Get physical address
pub const DMA_BUF_IOCTL_PHYS: u32 = 0x80086201;