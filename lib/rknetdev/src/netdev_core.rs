// SPDX-License-Identifier: BSD-3-Clause
// netdev_core.rs

// Authors: Simon Kuenzer <simon.kuenzer@neclab.eu>
//          Razvan Cojocaru <razvan.cojocaru93@gmail.com>
//          张子辰 <zichen350@gmail.com>

// Copyright (c) 2017 Intel Corporation
// Copyright (c) 2018, NEC Europe Ltd., NEC Corporation.
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
// 1. Redistributions of source code must retain the above copyright
//    notice, this list of conditions and the following disclaimer.
// 2. Redistributions in binary form must reproduce the above copyright
//    notice, this list of conditions and the following disclaimer in the
//    documentation and/or other materials provided with the distribution.
// 3. Neither the name of the copyright holder nor the names of its
//    contributors may be used to endorse or promote products derived from
//    this software without specific prior written permission.
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
// CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
// SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
// INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
// CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
// ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
// POSSIBILITY OF SUCH DAMAGE.
// Translated from unikraft/lib/uknetdev/include/uk/netdev_core.h.

use rklist::Tailq;
use rkalloc::RKalloc;
use runikraft::config;
#[cfg(feature="dispatcherthreads")]
use rksched::RKsched;
use super::netbuf::Netbuf;
use core::fmt::Display;
use core::ptr::NonNull;
use core::str::FromStr;
use core::ops::{Index,IndexMut};


pub type NetdevList<'a> = Tailq<'a,Netdev<'a>>;

//Ethernet size macros

//Header fields
pub const ETH_ADDR_LEN: usize = 6;
pub const ETH_TYPE_LEN: usize = 2;
pub const ETH_8021Q_LEN: usize = ETH_TYPE_LEN + 2;

//Ethernet header
//Untagged
pub const ETH_HDR_UNTAGGED_LEN: usize = 2*ETH_ADDR_LEN + ETH_TYPE_LEN;
//Single VLAN tag (IEEE 802.1q)
pub const ETH_HDR_8021Q_LEN: usize = ETH_HDR_UNTAGGED_LEN + ETH_8021Q_LEN;
//Double VLAN tag (IEEE 802.1q)
pub const ETH_HDR_8021AD_LEN: usize = ETH_HDR_UNTAGGED_LEN + 2*ETH_8021Q_LEN;

//Payload
pub const ETH_PAYLOAD_MAXLEN: usize = 1500;
pub const ETH_JPAYLOAD_MAXLEN: usize = 9000; // Jumbo frame.

//Frame sizes
pub const ETH_FRAME_MINLEN: usize = 60;

pub const ETH_FRAME_UNTAGGED_MAXLEN: usize = ETH_HDR_UNTAGGED_LEN +	
    ETH_PAYLOAD_MAXLEN;
pub const ETH_FRAME_8021Q_MAXLEN: usize = ETH_HDR_8021Q_LEN + 
    ETH_PAYLOAD_MAXLEN;
pub const ETH_FRAME_8021AD_MAXLEN: usize = ETH_HDR_8021AD_LEN + 
    ETH_PAYLOAD_MAXLEN;
pub const ETH_FRAME_MAXLEN: usize = ETH_FRAME_8021AD_MAXLEN;


pub const ETH_JFRAME_UNTAGGED_MAXLEN: usize = ETH_HDR_UNTAGGED_LEN +	
    ETH_JPAYLOAD_MAXLEN;
pub const ETH_JFRAME_8021Q_MAXLEN: usize = ETH_HDR_8021Q_LEN + 
    ETH_JPAYLOAD_MAXLEN;
pub const ETH_JFRAME_8021AD_MAXLEN: usize = ETH_HDR_8021AD_LEN + 
    ETH_JPAYLOAD_MAXLEN;
pub const ETH_JFRAME_MAXLEN: usize = ETH_JFRAME_8021AD_MAXLEN;


pub const NETDEV_HWADDR_LEN: usize = ETH_ADDR_LEN;
/// A structure used for Ethernet hardware addresses
#[repr(packed)]
pub struct Hwaddr {
    pub addr_bytes: [u8;NETDEV_HWADDR_LEN],
}

//The netdevice support rx/tx interrupt.
pub const FEATURE_RXQ_INTR_BIT: u32 = 0;
pub const FEATURE_RXQ_INTR_AVAILABLE: u32 = 1 << FEATURE_RXQ_INTR_BIT;
pub const FEATURE_TXQ_INTR_BIT: u32 = 1;
pub const FEATURE_TXQ_INTR_AVAILABLE: u32 = 1 << FEATURE_TXQ_INTR_BIT;

#[inline(always)]
pub fn rxintr_supported(feature: u32) -> bool {
    feature & FEATURE_RXQ_INTR_AVAILABLE != 0
}


/// A structure used to describe network device capabilities.
pub struct Info {
	pub max_rx_queues: u16,
	pub max_tx_queues: u16,
	pub in_queue_pairs: bool,   // If true, allocate queues in pairs.
	pub max_mtu: u16,           // Maximum supported MTU size.
	pub nb_encap_tx: u16,       // Number of bytes required as headroom for tx.
	pub nb_encap_rx: u16,       // Number of bytes required as headroom for rx.
	pub ioalign: u16,           // Alignment in bytes for packet data buffers
	pub features: u32,          // bitmap of the features supported
}

/// A structure used to describe device descriptor ring limitations.
pub struct QueueInfo {
	pub nb_max: u16,            // Max allowed number of descriptors.
	pub nb_min: u16,            // Min allowed number of descriptors.
	pub nb_align: u16,          // Number of descriptors should be aligned.
	pub nb_is_power_of_two: bool// Number of descriptors should be a power of two.
}

/// A structure used to configure a network device.
pub struct Conf {
    pub nb_rx_queues: u16,
    pub nb_tx_queues: u16,
}

//TODO
/**
 * @internal Queue structs that are defined internally by each driver
 * The datatype is introduced here for having type checking on the
 * API code
 */
pub struct TxQueue{}
pub struct RxQueue{}

/// Enum to describe possible states of an Runikraft network device.
pub enum State {
    Invalid,
    Unconfigured,
    Configured,
    Running,
}


/// Enum used by the extra information query interface.
/// 
/// The purpose of this type is to allow drivers to forward extra configurations
/// options such as IP information without parsing this data by themselves (e.g.,
/// strings of IP address and mask found on XenStore by netfront).
/// We do not want to introduce any additional parsing logic inside uknetdev API
/// because we assume that most network stacks provide this functionality
/// anyways. So one could forward this data within the glue code.
///
/// This list is extensible in the future without needing the drivers to adopt
/// any or all of the data types.
///
/// The extra information can available in one of the following formats:
/// - *_NINT16: Network-order raw int (4 bytes)
/// - *_STR: Null-terminated string
pub enum EinfoType {
	// IPv4 address and mask
	Ipv4AddrNint16,
	Ipv4AddrStr,
	Ipv4MaskNint16,
	Ipv4MaskStr,

	// IPv4 gateway
	Ipv4GwNint16,
	Ipv4GwStr,

	// IPv4 Primary DNS
	Ipv4Dns0Nint16,
	Ipv4Dns0Str,

	// IPv4 Secondary DNS
	Ipv4Dns1Nint16,
	Ipv4Dns1Str,
}

/// Function type used for queue event callbacks.
/// 
/// - `dev`: The Runikraft Network Device.
/// - `queue_id`: The queue on the Runikraft network device on which the event happened.
/// - `argp`: Extra argument that can be defined on callback registration.
pub type QueueEvent = fn(dev: *mut Netdev, queue_id: usize, argp: *mut u8);


/// User callback used by the driver to allocate netbufs
/// that are used to setup receive descriptors.
/// 
/// - `argp`: User-provided argument.
/// - `pkts`: Array for netbuf pointers that the function should allocate.
/// - `count`: Number of netbufs requested (equal to length of pkts).
/// - return: Number of successful allocated netbufs,
///  has to be in range `[0, count]`.
///  References to allocated packets are placed to `pkts[0]...pkts[count -1]`.
pub type AllocRxpkts = fn(argp: *mut u8, pkts: &mut[Netbuf], count: u16) -> u16;

/// A structure used to configure an Unikraft network device RX queue.
pub struct RxqueueConf {
	callback: QueueEvent,       // Event callback function.
	callback_cookie: *mut u8,   // Argument pointer for callback.

    a: *const dyn RKalloc,      // Allocator for descriptors.

	alloc_rxpkts: AllocRxpkts,  // Allocator for rx netbufs
	alloc_rxpkts_argp: *mut u8, // Argument for alloc_rxpkts
    #[cfg(feature="dispatcherthreads")]
    s: *const dyn RKsched,      // Scheduler for dispatcher.
}

/// A structure used to configure an Unikraft network device TX queue.
pub struct TxqueueConf {
    a: *const dyn RKalloc,      // Allocator for descriptors.
}

//Status code flags returned by rx and tx functions
///Successful operation (packet received or transmitted).
pub const STATUS_SUCCESS: usize = 0x1;
/// More room available for operation (e.g., still space on queue for sending
/// or more packets available on receive queue
pub const STATUS_MORE: usize = 0x2;
/// Queue underrun (e.g., out-of-memory when allocating new receive buffers).
pub const STATUS_UNDERRUN: usize = 0x4;

/// Driver callback type to retrieve one packet from a RX queue.
pub type RxOneFunc = fn(dev: &mut Netdev, queue: &mut RxQueue)->Option<*mut Netbuf>;

/// Driver callback type to submit one packet to a TX queue.
pub type TxOneFunc = fn(dev: &mut Netdev, queue: &mut RxQueue, pkt: *mut Netbuf)->Result<(),i32>;

///A trait containing the functions exported by a driver.
pub trait Operations {
    // RX queue interrupts.
    /// (optional) Driver callback type to enable interrupts of a RX queue
    fn rxq_intr_enable(&self, dev: &mut Netdev, queue: &mut RxQueue) -> Result<(),i32> {
        Err(0)
    }
    /// (optional) Driver callback type to disable interrupts of a RX queue
    fn rxq_intr_disable(&self, dev: &mut Netdev, queue: &mut RxQueue) -> Result<(),i32> {
        Err(0)
    }

    //Set/Get hardware address.
    /// (recommended) Driver callback type to get the hardware address.
    fn hwaddr_get(&self, dev: &Netdev)->Option<Hwaddr> {
        None
    }
    /// (optional) Driver callback type to set the hardware address.
    fn hwaddr_set(&self, dev: &mut Netdev, hwaddr: Hwaddr) -> Result<(),i32> {
        Err(0)
    }

    // Set/Get MTU.
    /// Driver callback type to get the MTU.
    fn mtu_get(&self, dev: &Netdev) -> u16;
    /// (optional) Driver callback type to set the MTU
    fn mtu_set(&self, dev: &mut Netdev, mtu: u16) -> Result<(),i32> {
        Err(0)
    }

    //Promiscuous mode.
    /// (optional) Driver callback type to enable or disable promiscuous mode
    fn promiscuous_set(&self, dev: &mut Netdev, mode: u32) -> Result<(),i32> {
        Err(0)
    }
    /// Driver callback type to get the current promiscuous mode
    fn promiscuous_get(&self, dev: &Netdev) -> u32;

    //Device/driver capabilities and info.
    /// Driver callback type to read device/driver capabilities,
    /// used for configuring the device
    fn info_get(&self, dev: &Netdev) -> Info;
    /// Driver callback type to retrieve TX queue limitations,
    /// used for configuring the TX queue later
    fn txq_info_get(&self, dev: &Netdev,queue_id: usize) -> Result<QueueInfo,i32>;
    /// Driver callback type to retrieve RX queue limitations,
    /// used for configuring the RX queue later
    fn rxq_info_get(&self, dev: &Netdev,queue_id: usize) -> Result<QueueInfo,i32>;
    ///
    fn einfo_get(&self, dev: &Netdev,econf: EinfoType)->Option<NonNull<u8>>{
        None
    }

    // Device life cycle.
    /// Driver callback type to configure a network device.
    fn configure(&self, dev: &mut Netdev, conf: Conf) -> Result<(),i32>;
    /// Driver callback type to set up a TX queue of an Runikraft network device.
    fn txq_configure(&self, dev: &mut Netdev, queue_id: usize, nb_desc: u16, tx_conf: TxqueueConf) -> Option<NonNull<TxQueue>>;
    /// Driver callback type to set up a RX queue of an Runikraft network device.
    fn rxq_configure(&self, dev: &mut Netdev, queue_id: usize, nb_desc: u16, rx_conf: RxqueueConf) -> Option<NonNull<RxQueue>>;
    /// Driver callback type to start a configured Runikraft network device.
    fn start(&self, dev: &mut Netdev)->Result<(),i32>;
}

/// Event handler configuration (internal to libuknetdev)
pub(crate) struct EventHandler {
    pub(crate) callback: QueueEvent,
    pub(crate) cookie: *mut u8,

    #[cfg(feature="dispatcherthreads")]
    pub(crate) events: Semaphore,  //semaphore to trigger events
    #[cfg(feature="dispatcherthreads")]
    pub(crate) dev: *mut Netdev,   //reference to net device
    #[cfg(feature="dispatcherthreads")]
    pub(crate) queue_id: usize,    //queue id which caused event
    #[cfg(feature="dispatcherthreads")]
    pub(crate) dispatcher: *const RkThread, //dispatcher thread
    #[cfg(feature="dispatcherthreads")]
    pub(crate) dispatcher_name: &'static str,//reference to thread name
    #[cfg(feature="dispatcherthreads")]
    pub(crate) dispatcher_s: *const Rksched,//Scheduler for dispatcher.
}

/// libuknetdev internal data associated with each network device.
pub(crate) struct Data<'a> {
    pub(crate) state: State,

    pub(crate) rxq_handler: [EventHandler;config::LIBUKNETDEV_MAXNBQUEUES],

    pub(crate) id: usize,//ID is assigned during registration
    pub(crate) drv_name: &'a str, 
}

pub struct Ipv4Addr {
    feild: [u8;4],
}

impl Ipv4Addr {
    pub fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self { feild: [a,b,c,d] }
    }
}

impl FromStr for Ipv4Addr {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = 0;
        let mut feild = [0,0,0,0];
        for x in s.split('.') {
            if i == 4 {
                return Err(());
            }
            feild[i] = match u8::from_str(x) {
                Ok(a) => a,
                Err(_) => {return Err(());}
            };
            i+=1;
        }
        if i == 4 {
            Ok(Self{feild})
        }
        else {
            Err(())
        }
    }
}

impl Index<usize> for Ipv4Addr {
    type Output = u8;
    fn index(&self, index: usize) -> &Self::Output {
        &self.feild[index]
    }
}

impl IndexMut<usize> for Ipv4Addr {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.feild[index]
    }
}

impl Display for Ipv4Addr {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}.{}.{}",self.feild[0],self.feild[1],self.feild[2],self.feild[3])
    }
}

pub struct Einfo {
    pub ipv4_addr: Ipv4Addr,
    pub ipv4_net_mask: Ipv4Addr,
    pub ipv4_gw_addr: Ipv4Addr,
}

/// A structure used to interact with a network device.
///
/// Function callbacks (tx_one, rx_one, ops) are registered by the driver before
/// registering the netdev. They change during device life time. Packet RX/TX
/// functions are added directly to this structure for performance reasons.
/// It prevents another indirection to ops.
pub struct Netdev<'a> {
    /// Packet transmission.
    pub(crate) tx_one: TxOneFunc,

    /// Packet reception.
    pub(crate) rx_one: RxOneFunc,

    /// Pointer to API-internal state data.
    pub(crate) data: Data<'a>,

    /// Functions callbacks by driver.
    pub(crate) ops: *const dyn Operations,

    /// Pointers to queues (API-private)
    pub(crate) rx_queue: [RxQueue;config::LIBUKNETDEV_MAXNBQUEUES],
    pub(crate) tx_queue: [TxQueue;config::LIBUKNETDEV_MAXNBQUEUES],

    /// Netdevice address configuration
    pub(crate) einfo: Einfo,

    pub(crate) scratch_pad: [u8;config::RK_NETDEV_SCRATCH_SIZE],
}
