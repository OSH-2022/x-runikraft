// test rkallocbuddy::RKalloc

#![no_std]
#![no_main]
extern crate rkalloc;
extern crate rkallocbuddy;
extern crate runikraft;

use rkalloc::RKalloc;
use rkallocbuddy::RKallocBuddy;
use runikraft::align_as;
use core::mem::{size_of, align_of};
use core::slice;

const HEAP_SIZE: usize = 65536;

static mut HEAP:align_as::A4096<[u8;HEAP_SIZE]> = align_as::A4096::new([0;HEAP_SIZE]);

#[no_mangle]
extern "C" fn rkplat_entry(_: i32, _: *mut *mut u8) -> ! {
    let arr_len = 10;
    unsafe {
        // test `RKallocBuddy::new()` and `RKallocBuddy::alloc()`
        let a = RKallocBuddy::new(HEAP.data.as_mut_ptr(), HEAP.data.len());
        let arr_heap = a.alloc(arr_len*size_of::<usize>(), align_of::<usize>());
        assert!(!arr_heap.is_null());
        let mut counter: usize = 0;
        let arr = slice::from_raw_parts_mut(arr_heap as *mut usize, arr_len as usize);
        while counter < arr_len {
            arr[counter] = (arr_len - counter) as usize;
            counter += 1;
        }
        counter = 0;
        while counter < arr_len {
            assert_eq!(arr[counter], (arr_len - counter) as usize);
            counter += 1;
        }
        
        // test `RKallocBuddy::realloc()`
        let new_arr_len = 20;
        let new_arr_heap = a.realloc(arr_heap, arr_len*size_of::<usize>(), new_arr_len*size_of::<usize>(), align_of::<usize>());
        assert!(!new_arr_heap.is_null());
        let new_arr = slice::from_raw_parts_mut(new_arr_heap as *mut usize, new_arr_len as usize);
        counter = 0;
        while counter < arr_len {
            assert_eq!(new_arr[counter], (arr_len - counter) as usize);
            counter += 1;
        }
        counter = 0;
        let mut data = 66;
        while counter < new_arr_len {
            new_arr[counter] = data;
            data += 7;
            counter += 1;
        }
        counter = 0;
        data = 66;
        while counter < new_arr_len {
            assert_eq!(new_arr[counter], data);
            data += 7;
            counter += 1;
        }

        // test `RKallocBuddy::dealloc()`
        a.dealloc(new_arr_heap, new_arr_len*size_of::<u8>(), align_of::<u8>());

        // test 'RKallocBuddy::alloc_zeroed()`
        let arr_len = 15;
        let arr_heap = a.alloc_zeroed(arr_len*size_of::<i8>(), align_of::<i8>());
        let arr = slice::from_raw_parts_mut(arr_heap as *mut i8, arr_len as usize);
        counter = 0;
        while counter < arr_len {
            assert_eq!(arr[counter], 0);
            counter += 1;
        }
        a.dealloc(arr_heap, arr_len*size_of::<i8>(), align_of::<i8>());
    }
    rkplat::println!("\nTest alloc_buddy0 passed!\n");
    rkplat::bootstrap::halt();
}
