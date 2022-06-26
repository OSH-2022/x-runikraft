// test rkalloc_buddy

// TODO
#![no_std]
#![no_main]
// mod tests{
extern crate rkalloc;
extern crate rkalloc_buddy;
extern crate runikraft;

use rkalloc::RKalloc;
use rkalloc_buddy::RKallocBuddy;
use runikraft::align_as;
use core::mem::{size_of, align_of};
use core::slice;

const HEAP_SIZE: usize = 65536;

static mut HEAP:align_as::A4096<[u8;HEAP_SIZE]> = align_as::A4096::new([0;HEAP_SIZE]);

#[no_mangle]
extern "C" fn rkplat_entry(_: i32, _: *mut *mut u8) -> ! {
    let arr_len = 10;
    unsafe {
        let a = RKallocBuddy::new(HEAP.data.as_mut_ptr(), HEAP.data.len());
        let arr_heap = a.alloc(arr_len*size_of::<usize>(), align_of::<usize>());
        let mut counter: usize = 0;
        let arr = slice::from_raw_parts_mut(arr_heap as *mut i32, arr_len as usize);
        loop {
            if counter < arr_len {
                arr[counter] = (10 - counter) as i32;
            } else {
                break;
            }
            counter += 1;
        }
        counter = 0;
        loop {
            if counter < arr_len {
                assert_eq!(arr[counter], (10 - counter) as i32);
            } else {
                break;
            }
            counter += 1;
        }
    }
    rkplat::println!("Test alloc_buddy0 passed!");
    rkplat::bootstrap::halt();
}
// }
