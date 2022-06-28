// SPDX-License-Identifier: MIT
// device_tree.rs

// Copyright (c) 2016 Marc Brinkmann <git@marcbrinkmann.de>
// Copyright (c) 2022 张子辰 <zichen350@gmail.com>

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

// Derived from mbr/device_tree-rs.

use core::mem::size_of;
use core::{str,slice};
use core::ptr::addr_of;
#[cfg(feature="driver_uart")]
use super::uart;
#[cfg(feature="driver_virtio")]
use super::virtio;
use rkalloc::{RKalloc,alloc_type};
use crate::console;
use crate::drivers::virtio::GPU_DEIVCE;

const MAGIC_NUMBER: u32 = 0xd00dfeed;
const SUPPORTED_VERSION: u32 = 17;
//const COMPAT_VERSION: u32 = 16;
const OF_DT_BEGIN_NODE: u32 = 0x00000001;
const OF_DT_END_NODE: u32 = 0x00000002;
const OF_DT_PROP: u32 = 0x00000003;
//const OF_DT_END: u32 = 0x00000009;

#[inline]
fn align(val: usize, to: usize) -> usize {
    val + (to - (val % to)) % to
}

#[derive(Debug)]
pub enum SliceReadError {
    UnexpectedEndOfInput,
}

pub type SliceReadResult<T> = Result<T, SliceReadError>;

pub trait SliceRead {
    fn read_be_u32(&self, pos: usize) -> SliceReadResult<u32>;
    fn read_be_u64(&self, pos: usize) -> SliceReadResult<u64>;
    fn read_bstring0(&self, pos: usize) -> SliceReadResult<&[u8]>;
    fn subslice(&self, start: usize, len: usize) -> SliceReadResult<&[u8]>;
}

impl<'a> SliceRead for &'a [u8] {
    fn read_be_u32(&self, pos: usize) -> SliceReadResult<u32> {
        // check size is valid
        if !(pos + 4 <= self.len()) {
            return Err(SliceReadError::UnexpectedEndOfInput);
        }

        Ok(
            (self[pos] as u32) << 24 | (self[pos + 1] as u32) << 16 | (self[pos + 2] as u32) << 8
                | (self[pos + 3] as u32),
        )
    }

    fn read_be_u64(&self, pos: usize) -> SliceReadResult<u64> {
        // check size is valid
        if !(pos + 8 <= self.len()) {
            return Err(SliceReadError::UnexpectedEndOfInput);
        }

        Ok(
            (self[pos] as u64) << 56 | (self[pos + 1] as u64) << 48 | (self[pos + 2] as u64) << 40
                | (self[pos + 3] as u64) << 32 | (self[pos + 4] as u64) << 24
                | (self[pos + 5] as u64) << 16 | (self[pos + 6] as u64) << 8
                | (self[pos + 7] as u64),
        )
    }

    fn read_bstring0(&self, pos: usize) -> SliceReadResult<&[u8]> {
        let mut cur = pos;
        while cur < self.len() {
            if self[cur] == 0 {
                return Ok(&self[pos..cur]);
            }

            cur += 1;
        }

        Err(SliceReadError::UnexpectedEndOfInput)
    }

    fn subslice(&self, start: usize, end: usize) -> SliceReadResult<&[u8]> {
        if !(end < self.len()) {
            return Err(SliceReadError::UnexpectedEndOfInput);
        }

        Ok(&self[start..end])
    }
}

#[derive(Debug)]
pub enum DeviceTreeError {
    /// The magic number `MAGIC_NUMBER` was not found at the start of the
    /// structure.
    InvalidMagicNumber,

    /// An offset or size found inside the device tree is outside of what was
    /// supplied to `load()`.
    SizeMismatch,

    /// Failed to read data from slice.
    SliceReadError(SliceReadError),

    /// The data format was not as expected at the given position
    ParseError(usize),

    /// While trying to convert a string that was supposed to be ASCII, invalid
    /// utf8 sequences were encounted
    Utf8Error,

    /// The device tree version is not supported by this library.
    VersionNotSupported,
}

#[derive(Debug)]
pub enum PropError {
    NotFound,
    Utf8Error,
    Missing0,
    SliceReadError(SliceReadError),
}

impl From<SliceReadError> for DeviceTreeError {
    fn from(e: SliceReadError) -> DeviceTreeError {
        DeviceTreeError::SliceReadError(e)
    }
}

impl From<str::Utf8Error> for DeviceTreeError {
    fn from(_: str::Utf8Error) -> DeviceTreeError {
        DeviceTreeError::Utf8Error
    }
}

//debug: addi    sp,sp,-720
//release: addi    sp,sp,-96
/// Load a device tree from a memory buffer.
pub fn parse(a: &dyn RKalloc, buffer: &[u8]) -> Result<(), DeviceTreeError> {
    //  0  magic_number: u32,

    //  4  totalsize: u32,
    //  8  off_dt_struct: u32,
    // 12  off_dt_strings: u32,
    // 16  off_mem_rsvmap: u32,
    // 20  version: u32,
    // 24  last_comp_version: u32,

    // // version 2 fields
    // 28  boot_cpuid_phys: u32,

    // // version 3 fields
    // 32  size_dt_strings: u32,

    // // version 17 fields
    // 36  size_dt_struct: u32,

    debug_assert_eq!(buffer.read_be_u32(0)?,MAGIC_NUMBER);
    debug_assert_eq!(buffer.read_be_u32(4)? as usize,buffer.len());

    // check version
    let version = buffer.read_be_u32(20)?;

    if version != SUPPORTED_VERSION {
        return Err(DeviceTreeError::VersionNotSupported);
    }

    let off_dt_struct = buffer.read_be_u32(8)? as usize;
    let off_dt_strings = buffer.read_be_u32(12)? as usize;
    let off_mem_rsvmap = buffer.read_be_u32(16)? as usize;
    //let boot_cpuid_phys = buffer.read_be_u32(28)?;

    // load reserved memory list
    let mut pos = off_mem_rsvmap;

    loop {
        //let offset = buffer.read_be_u64(pos)?;
        pos += 8;
        let size = buffer.read_be_u64(pos)?;
        pos += 8;

        if size == 0 {
            break;
        }
    }

    load_node(a, buffer, off_dt_struct, off_dt_strings)?;

    Ok(())
}

static mut SAVE_PROP_SPACE: [usize;size_of::<(&str,&[u8])>()*2] = [0;size_of::<(&str,&[u8])>()*2];

//debug: addi    sp,sp,-1152
//release: addi    sp,sp,-144
fn load_node(a: &dyn RKalloc, buffer: &[u8], start: usize, off_dt_strings: usize) -> Result<usize, DeviceTreeError> {
    // check for DT_BEGIN_NODE
    if buffer.read_be_u32(start)? != OF_DT_BEGIN_NODE {
        return Err(DeviceTreeError::ParseError(start));
    }

    let raw_name = buffer.read_bstring0(start + 4)?;
    let name = str::from_utf8(raw_name)?;

    // read all the props
    let mut pos = align(start + 4 + raw_name.len() + 1, 4);

    //我们需要临时储存设备属性，由于栈空间非常宝贵，这块区域必须位于静态区
    let props = unsafe{slice::from_raw_parts_mut(addr_of!(SAVE_PROP_SPACE) as *mut (&str,&[u8]),16)};
    let mut props_size = 0;

    while buffer.read_be_u32(pos)? == OF_DT_PROP {
        let val_size = buffer.read_be_u32(pos + 4)? as usize;
        let name_offset = buffer.read_be_u32(pos + 8)? as usize;

        // get value slice
        let val_start = pos + 12;
        let val_end = val_start + val_size;
        let val = buffer.subslice(val_start, val_end)?;

        // lookup name in strings table
        let prop_name = str::from_utf8(buffer.read_bstring0(off_dt_strings + name_offset)?)?;

        props[props_size] = (prop_name,val);
        props_size+=1;

        pos = align(val_end, 4);
    }
    parse_device(a, name, props, props_size)?;
    

    // finally, parse children
    while buffer.read_be_u32(pos)? == OF_DT_BEGIN_NODE {
        let new_pos = load_node(a, buffer, pos, off_dt_strings)?;
        pos = new_pos;
    }

    if buffer.read_be_u32(pos)? != OF_DT_END_NODE {
        return Err(DeviceTreeError::ParseError(pos));
    }

    pos += 4;

    Ok(pos)
}

fn parse_device(a: &dyn RKalloc, name: &str, props: &[(&str,&[u8])], props_size: usize) -> Result<(), DeviceTreeError> {
    if let Some(compatible) = prop_str(props, props_size, "compatible") {
        match compatible {
            #[cfg(feature="driver_uart")]
            "ns16550a" | "ns16550" => {
                unsafe {
                    if console::UART_DEIVCE.is_none() {
                        console::UART_DEIVCE = Some(&*alloc_type(a,uart::ns16550::Ns16550::new(name,
                            prop_u64(props,props_size,"reg").unwrap() as usize, 
                            prop_u32(props,props_size,"interrupts").unwrap() as usize)));
                    }
                }
            },
            #[cfg(feature="driver_goldfish_rtc")]
            "google,goldfish-rtc" => {
                unsafe {
                    if crate::time::RTC_DEVICE.is_none() {
                        crate::time::RTC_DEVICE = Some(&*alloc_type(a, 
                            super::rtc::goldfish::GoldfishRtc::new(name, prop_u64(props,props_size,"reg").unwrap() as usize)));
                    }
                }
            },
            #[cfg(feature="driver_virtio")]
            "virtio,mmio" => {
                let header = unsafe {
                    &mut *(prop_u64(props, props_size, "reg").
                    unwrap() as *mut virtio::VirtIOHeader)
                };
                println_bios!("Detected virtio device with vendor id {:#X}",header.vendor_id());
                match header.device_type() {
                    #[cfg(feature="driver_virtio_blk")]
                    virtio::DeviceType::Block => {todo!()},
                    #[cfg(feature="driver_virtio_console")]
                    virtio::DeviceType::Console => {todo!()},
                    #[cfg(feature="driver_virtio_gpu")]
                    virtio::DeviceType::GPU => {
                        unsafe {
                            if GPU_DEIVCE.is_none() {
                                GPU_DEIVCE = Some(&mut *alloc_type(a,virtio::gpu::VirtIOGpu::new(name,header).unwrap()));
                            }
                        }
                    },
                    #[cfg(feature="driver_virtio_input")]
                    virtio::DeviceType::Input => {todo!()},
                    #[cfg(feature="driver_virtio_net")]
                    virtio::DeviceType::Network => {todo!()},
                    t => println_bios!("WARNING: Unrecognized virtio device: {:?}",t),
                }
            },
            _ => {}
        }
    }
    Ok(())
}

fn prop_raw<'a>(props: &[(&str,&'a [u8])], props_size: usize, prop_name: &str) -> Option<&'a [u8]> {
    for i in 0..props_size {
        let val = props[i].1;
        if props[i].0 == prop_name {
            return Some(val);
        }
    }
    None
}

fn prop_str<'a>(props: &[(&str,&'a [u8])], props_size: usize, prop_name: &str) -> Option<&'a str> {
    prop_raw(props,props_size,prop_name).map(|val| {
        str::from_utf8(&val[0..(val.len()-1)]).unwrap()
    })
}

fn prop_u32(props: &[(&str,&[u8])], props_size: usize, prop_name: &str) -> Option<u32> {
    prop_raw(props,props_size,prop_name).map(|val| {
        val.read_be_u32(0).unwrap()
    })
}

fn prop_u64(props: &[(&str,&[u8])], props_size: usize, prop_name: &str) -> Option<u64> {
    prop_raw(props,props_size,prop_name).map(|val| {
        val.read_be_u64(0).unwrap()
    })
}

#[allow(unused)]
fn print_val(prop_name: &str, val: &[u8]) -> Result<(), DeviceTreeError>{
    match prop_name {
        "compatible" | "model" | "stdout-path" | "device_type" | "riscv,isa" | "mmu-type" => {
            println_bios!("{}={}",prop_name,str::from_utf8(&val[0..(val.len()-1)])?);
        },
        "interrupts" | "interrupt-parent" => {
            println_bios!("{}=0x{:x}",prop_name,val.read_be_u32(0)?);
        }
        "reg" => {
            print_bios!("{}=<",prop_name);
            for i in 0..val.len()/8 {
                print_bios!("0x{:x} ",val.read_be_u64(i*8)?);
            }
            println_bios!(">");
        }
        _ => {
            print_bios!("{}=<",prop_name);
            for i in val {
                print_bios!("0x{:x} ",i);
            }
            println_bios!(">");
        },
    }
    Ok(())
}

impl From<str::Utf8Error> for PropError {
    fn from(_: str::Utf8Error) -> PropError {
        PropError::Utf8Error
    }
}

impl From<SliceReadError> for PropError {
    fn from(e: SliceReadError) -> PropError {
        PropError::SliceReadError(e)
    }
}
