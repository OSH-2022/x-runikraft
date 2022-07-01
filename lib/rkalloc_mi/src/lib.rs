// SPDX-License-Identifier: MIT
// rkalloc_mi/lib.rs

// Authors: Daan Leijen
//          张子辰 <zichen350@gmail.com>

// Copyright (c) 2018-2021, Microsoft Research, Daan Leijen
// Copyright (C) 2022 吴骏东, 张子辰, 蓝俊玮, 郭耸霄 and 陈建绿.

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

#![no_std]
#![allow(unused)]

/// Minimal alignment necessary. On most platforms 16 bytes are needed
/// due to SSE registers for example. This must be at least `sizeof(void*)`
const MAX_ALIGN_SIZE: usize = 16;

#[cfg(target_pointer_width = "128")]
const PRT_SHIFT: usize = 4;
#[cfg(target_pointer_width = "64")]
const PRT_SHIFT: usize = 3;
#[cfg(target_pointer_width = "32")]
const PRT_SHIFT: usize = 2;

const PTR_SIZE: usize = 1<<PRT_SHIFT;
const PTR_SIZE_BITS: usize = 8*PTR_SIZE;
#[allow(non_upper_case_globals)]
const KiB: usize = 1024;
#[allow(non_upper_case_globals)]
const MiB: usize = 1024*1024;
#[allow(non_upper_case_globals)]
const GiB: usize = 1024*1024*1024;
/// maximum supported alignment is 1MiB
const ALIGNMENT_MAX: usize = 1024*1024;

const SMALL_WSIZE_MAX: usize = 128;
const SMALL_SIZE_MAX: usize = SMALL_WSIZE_MAX*PTR_SIZE;

const SEGMENT_SLICE_SHIFT: usize = 13+PRT_SHIFT; // 64KiB  (32KiB on 32-bit)
//64MiB on 64-bit, 4MiB on 32-bit
const SEGMENT_SHIFT: usize = if PTR_SIZE > 4 {10+SEGMENT_SLICE_SHIFT} else {7+SEGMENT_SLICE_SHIFT};
const SMALL_PAGE_SHIFT: usize = SEGMENT_SLICE_SHIFT; // 64KiB
const MEDIUM_PAGE_SHIFT: usize = 3 + SMALL_PAGE_SHIFT; // 512KiB

// Derived constants
const SEGMENT_SIZE: usize = 1<<SEGMENT_SHIFT;
const SEGMENT_ALIGN: usize = SEGMENT_SIZE;
const SEGMENT_MASK: usize = SEGMENT_SIZE - 1;
const SEGMENT_SLICE_SIZE: usize = 1<<SEGMENT_SLICE_SHIFT;
const SLICES_PER_SEGMENT: usize = SEGMENT_SIZE / SEGMENT_SLICE_SIZE; // 1024

const SMALL_PAGE_SIZE: usize = 1<<SMALL_PAGE_SHIFT;
const MEDIUM_PAGE_SIZE: usize = 1<<MEDIUM_PAGE_SHIFT;

const SMALL_OBJ_SIZE_MAX: usize = SMALL_PAGE_SIZE/4;    // 8KiB on 64-bit
const MEDIUM_OBJ_SIZE_MAX: usize = MEDIUM_PAGE_SIZE/4;  // 128KiB on 64-bit
const MEDIUM_OBJ_WSIZE_MAX: usize = MEDIUM_OBJ_SIZE_MAX/PTR_SIZE;
const LARGE_OBJ_SIZE_MAX: usize = SEGMENT_SIZE/2;       // 32MiB on 64-bit
const LARGE_OBJ_WSIZE_MAX: usize = LARGE_OBJ_SIZE_MAX/PTR_SIZE;

/// Maximum number of size classes. (spaced exponentially in 12.5% increments)
const BIN_HUGE: usize = 73;

/// Maximum slice offset (15)
const MAX_SLICE_OFFSET: usize = ALIGNMENT_MAX/SEGMENT_SLICE_SIZE - 1;

/// Used as a special value to encode block sizes in 32 bits.
const HUGE_BLOCK_SIZE: usize = 2*GiB;

/// blocks up to this size are always allocated aligned
const MAX_ALIGN_GUARANTEE: usize = 8*MAX_ALIGN_SIZE;

use core::{mem::size_of, sync::atomic::{AtomicUsize, AtomicPtr}};

use rkalloc::{RKalloc, RKallocState, RKallocExt};

type Encoded = usize;
//用read_tls获取
type ThreadID = usize;

/// free lists contain blocks
struct Block {
    next: *mut Block,
}

/// The delayed flags are used for efficient multi-threaded free-ing
enum Delayed {
    /// push on the owning heap thread delayed list
    UseDelayedFree = 0,
    /// temporary: another thread is accessing the owning heap
    DelayedFreeing = 1,
    /// optimize: push on page local thread free queue if another block is already in the heap thread delayed free list
    NoDelayedFree = 2,
    /// sticky, only resets on page reclaim
    NeverDelayedFree = 3,
}

bitflags::bitflags! {
    /// The `in_full` and `has_aligned` page flags are put in a union to efficiently
    /// test if both are false (`full_aligned == 0`) in the `mi_free` routine.
    struct PageFlags: u16 {
        const HAS_ALIGNED = 1<<0;
        const IN_FULL = 1<<1;
    }
}

/// Thread free list.
/// We use the bottom 2 bits of the pointer for Delayed flags
type ThreadFree = usize;


/// A page contains blocks of one specific size (`block_size`).
/// Each page has three list of free blocks:
/// `free` for blocks that can be allocated,
/// `local_free` for freed blocks that are not yet available to `mi_malloc`
/// `thread_free` for freed blocks by other threads
/// The `local_free` and `thread_free` lists are migrated to the `free` list
/// when it is exhausted. The separate `local_free` list is necessary to
/// implement a monotonic heartbeat. The `thread_free` list is needed for
/// avoiding atomic operations in the common case.
///
///
/// `used - |thread_free|` == actual blocks that are in use (alive)
/// `used - |thread_free| + |free| + |local_free| == capacity`
///
/// We don't count `freed` (as |free|) but use `used` to reduce
/// the number of memory accesses in the `mi_page_all_free` function(s).
///
/// Notes: 
/// - Access is optimized for `mi_free` and `mi_page_alloc` (in `alloc.c`)
/// - Using `uint16_t` does not seem to slow things down
/// - The size is 8 words on 64-bit which helps the page index calculations
///   (and 10 words on 32-bit, and encoded free lists add 2 words. Sizes 10 
///    and 12 are still good for address calculation)
/// - To limit the structure size, the `xblock_size` is 32-bits only; for 
///   blocks > MI_HUGE_BLOCK_SIZE the size is determined from the segment page size
/// - `thread_free` uses the bottom bits as a delayed-free flags to optimize
///   concurrent frees where only the first concurrent free adds to the owning
///   heap `thread_delayed_free` list (see `alloc.c:mi_free_block_mt`).
///   The invariant is that no-delayed-free is only set if there is
///   at least one block that will be added, or as already been added, to 
///   the owning heap `thread_delayed_free` list. This guarantees that pages
///   will be freed correctly even if only other threads free blocks.
struct Page {
    // "owned" by the segment
    /// slices in this page (0 if not a page)
    slice_count: u32,
    /// distance from the actual page data slice (0 if a page)
    slice_offset: u32,
    /// `true` if the page memory was reset
    is_reset: bool,
    /// `true` if the page virtual memory is committed
    is_committed: bool,
    /// `true` if the page was zero initialized
    is_zero_init: bool,

    // layout like this to optimize access in `mi_malloc` and `mi_free`
    /// number of blocks committed, must be the first field, see `segment.c:page_clear`
    capacity: u16,
    /// number of blocks reserved in memory
    reserved: u16,
    /// `in_full` and `has_aligned` flags (8 bits)
    flags: PageFlags,
    /// `true` if the blocks in the free list are zero initialized
    is_zero: bool,
    /// expiration count for retired blocks
    retire_expire: u8,

    /// list of available free blocks (`malloc` allocates from this list)
    free: *mut Block,
    /// two random keys to encode the free lists (see `_mi_block_next`)
    keys: [usize;2],
    /// number of blocks in use (including blocks in `local_free` and `thread_free`)
    used: u32,
    /// size available in each block (always `>0`) 
    xblock_size: u32,

    /// list of deferred free blocks by this thread (migrates to `free`)
    local_free: *mut Block,
    /// list of deferred free blocks freed by other threads
    xthread_free: AtomicUsize,
    xheap: AtomicUsize,

    /// next page owned by this thread with the same `block_size`
    next: *mut Page,
    /// previous page owned by this thread with the same `block_size`
    prev: *mut Page,
}

enum PageKind {
    /// small blocks go into 64KiB pages inside a segment
    Small, 
    /// medium blocks go into medium pages inside a segment
    Medium, 
    /// larger blocks go into a page of just one block
    Large, 
    /// huge blocks (> 16 MiB) are put into a single page in a single segment.
    Huge,
}

enum SegmentKind {
    // MI_SEGMENT_SIZE size with pages inside.
    Normal,
    // > MI_LARGE_SIZE_MAX segment with just one huge page inside.
    Huge,
}

// ------------------------------------------------------
// A segment holds a commit mask where a bit is set if
// the corresponding MI_COMMIT_SIZE area is committed.
// The MI_COMMIT_SIZE must be a multiple of the slice
// size. If it is equal we have the most fine grained 
// decommit (but setting it higher can be more efficient).
// The MI_MINIMAL_COMMIT_SIZE is the minimal amount that will
// be committed in one go which can be set higher than
// MI_COMMIT_SIZE for efficiency (while the decommit mask
// is still tracked in fine-grained MI_COMMIT_SIZE chunks)
// ------------------------------------------------------

const MINIMAL_COMMIT_SIZE: usize = 2*MiB;
const COMMIT_SIZE: usize = SEGMENT_SLICE_SIZE;// 64KiB
const COMMIT_MASK_BITS: usize = SEGMENT_SIZE / COMMIT_SIZE;
const COMMIT_MASK_FIELD_BITS: usize = PTR_SIZE_BITS;
const COMMIT_MASK_FIELD_COUNT: usize = COMMIT_MASK_BITS / COMMIT_MASK_FIELD_BITS;

struct CommitMask {
    mask: [usize;COMMIT_MASK_FIELD_COUNT],
}

type Slice=Page;
type MSecs = u64;

/// Segments are large allocated memory blocks (8mb on 64 bit) from
/// the OS. Inside segments we allocated fixed size _pages_ that
/// contain blocks.
struct Segment {
    /// memory id for arena allocation
    memid: usize,
    /// `true` if we cannot decommit/reset/protect in this memory (i.e. when allocated using large OS pages) 
    mem_is_pinned: bool,
    /// in large/huge os pages?
    mem_is_large: bool,
    /// `true` if the whole segment is eagerly committed
    mem_is_committed: bool,

    allow_decommit: bool,
    decommit_expire: MSecs,
    decommit_mask: CommitMask,
    commit_mask: CommitMask,

    abandoned_next: AtomicPtr<Segment>,

    // from here is zero initialized
    /// the list of freed segments in the cache (must be first field, see `segment.c:mi_segment_init`)
    next: *mut Segment,

    /// abandoned pages (i.e. the original owning thread stopped) (`abandoned <= used`)
    abandoned: usize,
    /// count how often this segment is visited in the abandoned list (to force reclaim it it is too long)
    abandoned_visits: usize,
    /// count of pages in use
    used: usize,
    /// verify addresses in debug mode: `mi_ptr_cookie(segment) == segment->cookie`  
    cookie: usize,

    /// for huge segments this may be different from `SLICES_PER_SEGMENT`
    segment_slices: usize,
    /// initial slices we are using segment info and possible guard pages
    segment_info_slices: usize,

    // layout like this to optimize access in `mi_free`
    kind: SegmentKind,
    ///  unique id of the thread owning this segment
    thread_id: AtomicUsize,
    /// entries in the `slices` array, at most `SLICES_PER_SEGMENT`
    slice_entries: usize,
    slices: [Slice;SLICES_PER_SEGMENT],
}

// ------------------------------------------------------
// Heaps
// Provide first-class heaps to allocate from.
// A heap just owns a set of pages for allocation and
// can only be allocate/reallocate from the thread that created it.
// Freeing blocks can be done from any thread though.
// Per thread, the segments are shared among its heaps.
// Per thread, there is always a default heap that is
// used for allocation; it is initialized to statically
// point to an empty heap to avoid initialization checks
// in the fast path.
// ------------------------------------------------------

/// Pages of a certain block size are held in a queue.
struct PageQueue {
    first: *mut Page,
    last: *mut Page,
    block_size: usize,
}

const BIN_FULL:usize = BIN_HUGE+1;

/// Random context
struct RandomCxt {
    input: [u32;16],
    output: [u32;16],
    output_available: i32,
}

/// In debug mode there is a padding structure at the end of the blocks to check for buffer overflows
#[cfg(debug_assertions)]
struct Padding {
    /// encoded block value to check validity of the padding (in case of overflow)
    canary: u32,
    /// padding bytes before the block. (mi_usable_size(p) - delta == exact allocated bytes)
    delta: u32,
}
#[cfg(debug_assertions)]
const PADDING_SIZE: usize = size_of::<Padding>();
#[cfg(debug_assertions)]
const PADDING_WSIZE: usize = (PADDING_SIZE+PTR_SIZE-1)/PTR_SIZE;

#[cfg(not(debug_assertions))]
const PADDING_SIZE: usize = 0;
#[cfg(not(debug_assertions))]
const PADDING_WSIZE: usize = 0;

const PAGES_DIRECT: usize = SMALL_WSIZE_MAX + PADDING_WSIZE + 1;

/// A heap owns a set of pages.
struct Heap {
    tld: *mut Tld,
    /// optimize: array where every entry points a page with possibly free blocks in the corresponding queue for that size.
    pages_free_direct: [*mut Page;PAGES_DIRECT],
    /// queue of pages for each size class (or "bin")
    pages: [PageQueue; BIN_FULL + 1],
    thread_delayed_free: AtomicPtr<Block>,
    /// thread this heap belongs too
    thread_id: ThreadID,
    /// random cookie to verify pointers (see `_mi_ptr_cookie`)
    cookie: usize,
    /// two random keys used to encode the `thread_delayed_free`
    keys: [usize;2],
    /// random number context used for secure allocation
    random: RandomCxt,
    /// total number of pages in the `pages` queues.
    page_count: usize,
    ///  smallest retired index (retired pages are fully free, but still in the page queues)
    page_retired_min: usize,
    /// largest retired index into the `pages` array.
    page_retired_max: usize,
    /// list of heaps per thread
    next: *mut Heap,
    /// `true` if this heap should not reclaim abandoned pages
    no_reclaim: bool,
}

macro_rules! internal_assert {
    ($($arg:tt)*) => (if cfg!(debug_lv2) { assert!($($arg)*); })
}

macro_rules! expensive_assert {
    ($($arg:tt)*) => (if cfg!(debug_lv3) { assert!($($arg)*); })
}

// ------------------------------------------------------
// Statistics
// ------------------------------------------------------
struct StatCount {
    allocated: i64,
    freed: i64,
    peak: i64,
    current: i64,
}

struct StatCounter {
    total: i64,
    count: i64,
}

struct Stat {
    segments: StatCount,
    pages: StatCount,
    reserved: StatCount,
    committed: StatCount,
    reset: StatCount,
    page_committed: StatCount,
    segments_abandoned: StatCount,
    pages_abandoned: StatCount,
    threads: StatCount,
    normal: StatCount,
    huge: StatCount,
    large: StatCount,
    malloc: StatCount,
    segments_cache: StatCount,
    pages_extended: StatCounter,
    mmap_calls: StatCounter,
    commit_calls: StatCounter,
    page_no_retire: StatCounter,
    searches: StatCounter,
    normal_count: StatCounter,
    huge_count: StatCounter,
    large_count: StatCounter,
    #[cfg(feature="stat_lv2")]
    normal_bins: [StatCount;BIN_HUGE+1],
}

impl StatCount {
    fn increse(&mut self, amount: usize) {
        if cfg!(stat_lv1){
            todo!()
        }
    }
    fn decrease(&mut self, amount: usize) {
        if cfg!(stat_lv1){
            todo!()
        }
    }
}

impl StatCounter {
    fn increase(&mut self, amount: usize) {
        if cfg!(stat_lv1){
            todo!()
        }
    }
}


// ------------------------------------------------------
// Thread Local data
// ------------------------------------------------------

/// A "span" is is an available range of slices. The span queues keep
/// track of slice spans of at most the given `slice_count` (but more than the previous size class).
struct SpanQueue {
    first: *mut Slice,
    last: *mut Slice,
    slice_count: usize,
}

// 35 == mi_segment_bin(MI_SLICES_PER_SEGMENT)
const SEGMENT_BIN_MAX: usize = 35;

/// OS thread local data
struct OsTld {
    /// start point for next allocation
    region_idx: usize,
    /// points to tld stats
    stats: Stat
}

/// Segments thread local data
struct SegmentsTld {
    /// free slice spans inside segments
    spans: [SpanQueue;SEGMENT_BIN_MAX+1],
    /// current number of segments;
    count: usize,
    /// peak number of segments
    peak_count: usize,
    /// current size of all segments
    current_size: usize,
    /// peak size of all segments
    peak_size: usize,
    /// points to tld stats
    stats: *mut Stat,
    /// points to os stats
    os: *mut OsTld,
}

/// Thread local data
struct Tld {
    /// monotonic heartbeat count
    heartbeat: u64,
    /// true if deferred was called; used to prevent infinite recursion.
    recurse: bool,
    /// backing heap of this thread (cannot be deleted)
    heap_backing: *mut Heap,
    /// list of heaps in this thread (so we can abandon all when the thread terminates)
    heaps: *mut Heap,
    /// segment tld
    segments: SegmentsTld,
    /// os tld
    os: OsTld,
    /// statistics
    stats: Stat,
}

pub struct RKallocMi {

    //状态信息
    size_data: usize,    
    size_left: usize,       //剩余可用空间大小
    size_total: usize,      //总可用空间大小
}

unsafe impl RKalloc for RKallocMi {
    unsafe fn alloc(&self, size: usize, align: usize) -> *mut u8 {
        todo!()
    }
    unsafe fn realloc(&self, old_ptr: *mut u8, old_size: usize, new_size: usize, align: usize) -> *mut u8 {
        todo!()
    }
    unsafe fn dealloc(&self, ptr: *mut u8, size: usize, align: usize) {
        todo!()
    }
}

unsafe impl RKallocExt for RKallocMi {
    unsafe fn dealloc_ext(&self, ptr: *mut u8) {
        todo!()
    }
    unsafe fn realloc_ext(&self, old_ptr: *mut u8, new_size: usize) -> *mut u8 {
        todo!()
    }
}

impl RKallocState for RKallocMi {
    fn free_size(&self) -> usize {
        self.size_left
    }
    fn total_size(&self) -> usize {
        self.size_total
    }
}
