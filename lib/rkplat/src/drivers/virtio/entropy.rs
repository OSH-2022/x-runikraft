// SPDX-License-Identifier: MIT
// Authors: 张子辰 <zichen350@gmail.com>

// Copyright (c) 2019-2020 rCore Developers
// Copyright (c) 2022 张子辰 <zichen350@gmail.com>

use super::*;
use bitflags::*;
use core::hint::spin_loop;
use log::*;

/// The virtio entropy device, aka random number generator (rng), supplies 
/// high-quality randomness for guest use.
pub struct VirtIOEntropy<'a> {
    header: &'static mut VirtIOHeader,
    recv_queue: VirtQueue<'a>,
    name: [u8;32],
    name_size: usize,
}

impl VirtIOEntropy<'_> {
    /// Create a new VirtIO-Net driver.
    pub fn new(name: &str, header: &'static mut VirtIOHeader) -> Result<Self> {
        header.begin_init(|features| {
            let features = Features::from_bits_truncate(features);
            info!("Device features {:?}", features);
            let supported_features = Features::empty();
            (features & supported_features).bits()
        });

        let queue_num = 2; // for simplicity
        let recv_queue = VirtQueue::new(header, QUEUE_RECEIVE, queue_num)?;

        header.finish_init();

        let mut name1: [u8;32] = [0;32];
        for i in 0..name.len() {
            name1[i] = name.as_bytes()[i];
        }

        Ok(Self {
            header,
            recv_queue,
            name: name1,
            name_size: name.len()
        })
    }

    /// Acknowledge interrupt.
    pub fn ack_interrupt(&mut self) -> bool {
        self.header.ack_interrupt()
    }

    /// Whether can receive packet.
    pub fn can_recv(&self) -> bool {
        self.recv_queue.can_pop()
    }

    /// Receive a packet.
    pub fn recv(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.recv_queue.add(&[], &[buf])?;
        self.header.notify(QUEUE_RECEIVE as u32);
        while !self.recv_queue.can_pop() {
            spin_loop();
        }

        let (_, len) = self.recv_queue.pop_used()?;
        // let header = unsafe { header.assume_init() };
        Ok(len as usize)
    }
}

impl crate::drivers::Device for VirtIOEntropy<'_> {
    fn name<'a>(&'a self) -> &'a str {
        unsafe {core::str::from_utf8_unchecked(core::slice::from_raw_parts(self.name.as_ptr(), self.name_size))}
    }
}

bitflags! {
    struct Features: u64 {
        // device independent
        const NOTIFY_ON_EMPTY       = 1 << 24; // legacy
        const ANY_LAYOUT            = 1 << 27; // legacy
        const RING_INDIRECT_DESC    = 1 << 28;
        const RING_EVENT_IDX        = 1 << 29;
        const UNUSED                = 1 << 30; // legacy
        const VERSION_1             = 1 << 32; // detect legacy
    }
}

const QUEUE_RECEIVE: usize = 0;
