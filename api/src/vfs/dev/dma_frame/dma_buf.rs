use core::any::Any;

use axfs_ng_vfs::{DeviceId, NodeFlags, VfsError, VfsResult};
use starry_vm::VmMutPtr;

use crate::vfs::DeviceOps;

/// Device ID for dma-buf devices
pub const DMA_BUF_DEVICE_ID: DeviceId = DeviceId::new(253, 0);

/// DMA buffer device
pub struct DmaBuf {
    /// The size of the DMA buffer
    size: usize,
    /// CPU virtual address
    cpu_addr: usize,
    /// Bus address for device access
    bus_addr: u64,
}

impl DmaBuf {
    /// Creates a new DMA buffer device.
    pub fn new(size: usize, cpu_addr: usize, bus_addr: u64) -> Self {
        warn!("dma_buf: Creating new DmaBuf instance, size={}, cpu_addr={:#x}, bus_addr={:#x}", 
              size, cpu_addr, bus_addr);
        Self {
            size,
            cpu_addr,
            bus_addr,
        }
    }
}

impl DeviceOps for DmaBuf {
    fn read_at(&self, buf: &mut [u8], offset: u64) -> VfsResult<usize> {
        warn!("dma_buf: read_at called, offset={}, buf_len={}", offset, buf.len());
        // In a full implementation, we would read from the DMA buffer
        // For now, we just return an error as dma-buf devices are not meant to be read directly
        Err(VfsError::InvalidInput)
    }

    fn write_at(&self, buf: &[u8], offset: u64) -> VfsResult<usize> {
        warn!("dma_buf: write_at called, offset={}, buf_len={}", offset, buf.len());
        // In a full implementation, we would write to the DMA buffer
        // For now, we just return an error as dma-buf devices are not meant to be written directly
        Err(VfsError::InvalidInput)
    }

    fn ioctl(&self, cmd: u32, arg: usize) -> VfsResult<usize> {
        warn!("dma_buf: ioctl called cmd={:#x}, arg={:#x}", cmd, arg);
        // Handle dma-buf specific ioctls
        match cmd {
            // For now, we just return success for all ioctls
            // In a full implementation, we would handle specific dma-buf ioctls
            _ => {
                // Best-effort: if arg is a user pointer, zero the first u32 there so
                // user-space doesn't read uninitialized memory
                if arg != 0 {
                    if let Err(e) = (arg as *mut u32).vm_write(0u32) {
                        warn!("dma_buf: ioctl vm_write failed: {:?}", e);
                        return Err(VfsError::InvalidInput);
                    }
                }
                Ok(0)
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        warn!("dma_buf: as_any called");
        self
    }

    fn flags(&self) -> NodeFlags {
        warn!("dma_buf: flags called");
        NodeFlags::NON_CACHEABLE
    }
}