// SPDX-License-Identifier: MIT

// Authors: Jiajie Chen <noc@jiegec.ac.cn>
//          Runji Wang <wangrunji0408@163.com>
//          Yuekai Jia <equation618@gmail.com>

// Copyright (c) 2019-2020 rCore Developers

// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:

// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.

// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! VirtIO guest drivers.

#![deny(unused_must_use)]
#![allow(clippy::identity_op)]
#![allow(dead_code)]

// #[macro_use]
extern crate log;

extern crate alloc;

#[cfg(feature = "driver_virtio_blk")]
mod blk;
#[cfg(feature = "driver_virtio_console")]
mod console;
#[cfg(feature = "driver_virtio_gpu")]
mod gpu;
mod hal;
mod header;
#[cfg(feature = "driver_virtio_input")]
mod input;
#[cfg(feature = "driver_virtio_net")]
mod net;
mod queue;
#[cfg(feature = "driver_virtio_entropy")]
mod entropy;

#[cfg(feature = "driver_virtio_blk")]
pub use self::blk::{BlkResp, RespStatus, VirtIOBlk};
#[cfg(feature = "driver_virtio_console")]
pub use self::console::VirtIOConsole;
#[cfg(feature = "driver_virtio_gpu")]
pub use self::gpu::VirtIOGpu;
pub use self::header::*;
#[cfg(feature = "driver_virtio_input")]
pub use self::input::{InputConfigSelect, InputEvent, VirtIOInput};
#[cfg(feature = "driver_virtio_net")]
pub use self::net::VirtIONet;
#[cfg(feature = "driver_virtio_entropy")]
pub use self::entropy::VirtIOEntropy;
use self::queue::VirtQueue;
use core::mem::size_of;
use hal::*;

const PAGE_SIZE: usize = 0x1000;

/// The type returned by driver methods.
type Result<T = ()> = core::result::Result<T, Error>;

// pub struct Error {
//     kind: ErrorKind,
//     reason: &'static str,
// }

/// The error type of VirtIO drivers.
#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// The buffer is too small.
    BufferTooSmall,
    /// The device is not ready.
    NotReady,
    /// The queue is already in use.
    AlreadyUsed,
    /// Invalid parameter.
    InvalidParam,
    /// Failed to alloc DMA memory.
    DmaError,
    /// I/O Error
    IoError,
    ///
    Uninitialized,
}

/// Align `size` up to a page.
fn align_up(size: usize) -> usize {
    (size + PAGE_SIZE) & !(PAGE_SIZE - 1)
}

/// Pages of `size`.
fn pages(size: usize) -> usize {
    (size + PAGE_SIZE - 1) / PAGE_SIZE
}

/// Convert a struct into buffer.
unsafe trait AsBuf: Sized {
    fn as_buf(&self) -> &[u8] {
        unsafe { core::slice::from_raw_parts(self as *const _ as _, size_of::<Self>()) }
    }
    fn as_buf_mut(&mut self) -> &mut [u8] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut _ as _, size_of::<Self>()) }
    }
}


#[cfg(feature = "driver_virtio_gpu")]
pub static mut __GPU_DEIVCE: Option<&'static mut VirtIOGpu> = None;

#[cfg(feature = "driver_virtio_input")]
pub static mut __INPUT_DEIVCE: Option<&'static mut VirtIOInput> = None;

#[cfg(feature = "driver_virtio_entropy")]
pub static mut __ENTROPY_DEIVCE: Option<&'static mut VirtIOEntropy> = None;
