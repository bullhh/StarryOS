use core::any::Any;

use axfs_ng_vfs::{DeviceId, NodeFlags, VfsError, VfsResult};
use starry_vm::{VmMutPtr, VmPtr};

use crate::vfs::DeviceOps;

// Import ArceOS DMA allocator
use axdma::{alloc_coherent, dealloc_coherent};
use core::alloc::Layout;

// Include the DMA heap definitions
use crate::vfs::dev::dma_frame::dma_heap_defs::*;

/// Device ID for /dev/dma_heap/system
pub const DMA_HEAP_SYSTEM_DEVICE_ID: DeviceId = DeviceId::new(252, 0);

/// DMA heap system device
pub struct DmaHeapSystem;

impl DmaHeapSystem {
    /// Creates a new DMA heap system device.
    pub fn new() -> Self {
        warn!("dma_heap: Creating new DmaHeapSystem instance");
        Self
    }
}

impl Default for DmaHeapSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceOps for DmaHeapSystem {
    fn read_at(&self, _buf: &mut [u8], _offset: u64) -> VfsResult<usize> {
        warn!("dma_heap: read_at called");
        // DMA heap devices are not meant to be read directly
        Err(VfsError::InvalidInput)
    }

    fn write_at(&self, _buf: &[u8], _offset: u64) -> VfsResult<usize> {
        warn!("dma_heap: write_at called");
        // DMA heap devices are not meant to be written directly
        Err(VfsError::InvalidInput)
    }

    fn ioctl(&self, cmd: u32, arg: usize) -> VfsResult<usize> {
        warn!("dma_heap: ioctl called cmd={:#x}, arg={:#x}", cmd, arg);
        
        // Handle common DMA heap ioctls
        match cmd {
            DMA_HEAP_IOCTL_ALLOC => {
                warn!("dma_heap: handling DMA_HEAP_IOCTL_ALLOC");
                if arg == 0 {
                    return Err(VfsError::InvalidInput);
                }
                
                // Read the allocation data from user space
                let alloc_data = match (arg as *const DmaHeapAllocationData).vm_read() {
                    Ok(data) => data,
                    Err(e) => {
                        warn!("dma_heap: ioctl vm_read failed: {:?}", e);
                        return Err(VfsError::InvalidInput);
                    }
                };
                
                warn!("dma_heap: requested allocation size={}", alloc_data.len);
                
                // Allocate DMA coherent memory using ArceOS allocator
                let layout = Layout::from_size_align(alloc_data.len as usize, 4096)
                    .map_err(|_| VfsError::InvalidInput)?;
                
                let dma_info = unsafe { alloc_coherent(layout) }
                    .map_err(|_| VfsError::NoMemory)?;
                
                warn!("dma_heap: allocated DMA memory, cpu_addr={:#x}, bus_addr={:#x}", 
                      dma_info.cpu_addr.as_ptr() as usize, 
                      dma_info.bus_addr.as_u64());
                
                // For now, we'll just log the allocation and return success
                // In a full implementation, we would create a dma-buf file descriptor
                // and return it to user space through the alloc_data.fd field
                let mut result_data = alloc_data;
                result_data.fd = 0; // Placeholder fd
                
                // Write the updated allocation data back to user space
                if let Err(e) = (arg as *mut DmaHeapAllocationData).vm_write(result_data) {
                    warn!("dma_heap: ioctl vm_write failed: {:?}", e);
                    // Clean up the allocated memory
                    unsafe { dealloc_coherent(dma_info, layout) };
                    return Err(VfsError::InvalidInput);
                }
                
                Ok(0)
            }
            // For other ioctls, return success but log them
            _ => {
                warn!("dma_heap: handling unknown ioctl cmd={:#x}", cmd);
                // Best-effort: if arg is a user pointer, zero the first u32 there so
                // user-space doesn't read uninitialized memory
                if arg != 0 {
                    // write a safe default (0) to the user pointer
                    if let Err(e) = (arg as *mut u32).vm_write(0u32) {
                        warn!("dma_heap: ioctl vm_write failed: {:?}", e);
                        return Err(VfsError::InvalidInput);
                    }
                }
                Ok(0)
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        warn!("dma_heap: as_any called - used for dynamic type checking");
        self
    }

    fn flags(&self) -> NodeFlags {
        warn!("dma_heap: flags called - returning NON_CACHEABLE flag");
        NodeFlags::NON_CACHEABLE
    }
}