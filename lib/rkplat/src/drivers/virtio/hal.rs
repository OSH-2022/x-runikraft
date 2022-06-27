// SPDX-License-Identifier: MIT
// Authors: Jiajie Chen <noc@jiegec.ac.cn>
//          Runji Wang <wangrunji0408@163.com>
//          Yuekai Jia <equation618@gmail.com>
// Copyright (c) 2019-2020 rCore Developers

use super::*;

type VirtAddr = usize;
type PhysAddr = usize;

pub struct DMA {
    paddr: u32,
    pages: u32,
}

impl DMA {
    pub fn new(pages: usize) -> Result<Self> {
        let paddr = unsafe { __rkplat_virtio_dma_alloc(pages) };
        if paddr == 0 {
            return Err(Error::DmaError);
        }
        Ok(DMA {
            paddr: paddr as u32,
            pages: pages as u32,
        })
    }

    pub fn paddr(&self) -> usize {
        self.paddr as usize
    }

    pub fn vaddr(&self) -> usize {
        phys_to_virt(self.paddr as usize)
    }

    /// Page frame number
    pub fn pfn(&self) -> u32 {
        self.paddr >> 12
    }

    /// Convert to a buffer
    pub unsafe fn as_buf(&self) -> &'static mut [u8] {
        core::slice::from_raw_parts_mut(self.vaddr() as _, PAGE_SIZE * self.pages as usize)
    }
}

impl Drop for DMA {
    fn drop(&mut self) {
        let err = unsafe { __rkplat_virtio_dma_dealloc(self.paddr as usize, self.pages as usize) };
        assert_eq!(err, 0, "failed to deallocate DMA");
    }
}

pub fn phys_to_virt(paddr: PhysAddr) -> VirtAddr {
    unsafe { __rkplat_virtio_phys_to_virt(paddr) }
}

pub fn virt_to_phys(vaddr: VirtAddr) -> PhysAddr {
    unsafe { __rkplat_virtio_virt_to_phys(vaddr) }
}

// rkplat应该提供这4个函数
extern "C" {
    fn __rkplat_virtio_dma_alloc(pages: usize) -> PhysAddr;
    fn __rkplat_virtio_dma_dealloc(paddr: PhysAddr, pages: usize) -> i32;
    fn __rkplat_virtio_phys_to_virt(paddr: PhysAddr) -> VirtAddr;
    fn __rkplat_virtio_virt_to_phys(vaddr: VirtAddr) -> PhysAddr;
}
