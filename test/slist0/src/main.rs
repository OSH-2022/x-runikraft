// test rkalloc_buddy

// TODO
#![no_std]
#![no_main]

extern crate rkboot;
extern crate runikraft;
extern crate rkalloc;

use runikraft::compat_list::*;
use rkalloc::*;
use core::mem::{size_of, align_of};
use core::slice;
use core::ptr::NonNull;

#[no_mangle]
fn main(_args: &mut [&str])->i32 {
    let mut slist_a = Slist::<i32>::new();
    unsafe{
        let a = rkalloc::get_default().expect("error: fail to find global allocator\n");
        let mut arr_len = 15;
        let arr_heap = a.alloc(arr_len*size_of::<i32>(), align_of::<i32>());
        assert!(!arr_heap.is_null());
        let mut counter: usize = 0;
        let arr = slice::from_raw_parts_mut(arr_heap as *mut i32, arr_len as usize);
        while counter < arr_len {
            arr[counter] = (arr_len - counter) as i32;
            counter += 1;
        }

        // test `SList::push_front()`, `SlistNode::insert_after()` and `Slist::iter()`
        counter = 0;
        while counter < arr_len {
            let ptr_e = alloc_type::<SlistNode<i32>>(a, SlistNode::<i32>::new(arr[counter]));
            let mut node1 = NonNull::new(ptr_e).expect("error: fail to get node\n");
            slist_a.push_front(node1);
            counter += 1;
            if counter < arr_len {
                let ptr_e = alloc_type::<SlistNode<i32>>(a, SlistNode::<i32>::new(arr[counter]));
                let mut node2 = NonNull::new(ptr_e).expect("error: fail to get node\n");
                node1.as_mut().insert_after(node2);
                counter += 1;
            }
        }
        let slist_iter = slist_a.iter();
        let result = [1, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14];
        counter = 0;
        for node in slist_iter {
            // rkplat::println!("counter: {}, result in node: {}, expect result: {}", counter, node.as_ref().element, result[counter]);
            assert_eq!(node.as_ref().element, result[counter]);
            counter += 1;
        }

        // test `SlistNode::remove_after()`
        let slist_iter = slist_a.iter();
        let result = [1, 2, 5, 7, 9, 11, 13, 15];
        counter = 0;
        for mut node in slist_iter {
            if node.as_ref().element % 2 == 1 {
                node.as_mut().remove_after(Some(&mut slist_a));
                a.dealloc(node.as_ptr() as *mut u8, size_of::<SlistNode<i32>>(), align_of::<SlistNode<i32>>());
            }
        }
        let slist_iter = slist_a.iter();
        for node in slist_iter {
            assert_eq!(node.as_ref().element, result[counter]);
            counter += 1;
        }
        arr_len = 8;

        // test `Slist::pop_front()` and `Slist::is_empty()`
        counter = 0;
        while counter < arr_len {
            let node = slist_a.pop_front().expect("error: fail to get node from pop_front()\n");
            let e = node.as_ref().element;
            assert_eq!(e, result[counter]);
            a.dealloc(node.as_ptr() as *mut u8, size_of::<SlistNode<i32>>(), align_of::<SlistNode<i32>>());
            counter += 1;
        }
        assert!(slist_a.is_empty());
    }
    rkplat::println!("\nTest slist0 passed!\n");
    return 0;
}