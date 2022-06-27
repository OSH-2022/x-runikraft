// test compact_list-stailq

#![no_std]
#![no_main]
#![allow(unused_assignments)]

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
    let mut stailq_a = Stailq::<i32>::new();
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

        // test `Stailq::push_front()`, `StailqNode::insert_after()` and `Stailq::iter()`
        counter = 0;
        while counter < arr_len {
            let ptr_e = alloc_type::<StailqNode<i32>>(a, StailqNode::<i32>::new(arr[counter]));
            let mut node1 = NonNull::new(ptr_e).expect("error: fail to get node\n");
            stailq_a.push_front(node1);
            counter += 1;
            if counter < arr_len {
                let ptr_e = alloc_type::<StailqNode<i32>>(a, StailqNode::<i32>::new(arr[counter]));
                let node2 = NonNull::new(ptr_e).expect("error: fail to get node\n");
                node1.as_mut().insert_after(node2, Some(&mut stailq_a));
                counter += 1;
            }
        }
        let result = [1, 3, 2, 5, 4, 7, 6, 9, 8, 11, 10, 13, 12, 15, 14];
        counter = 0;
        for node in stailq_a.iter() {
            // rkplat::println!("counter: {}, result in node: {}, expect result: {}", counter, node.as_ref().element, result[counter]);
            assert_eq!(node.element, result[counter]);
            counter += 1;
        }

        // test `StailqNode::remove_after()`
        let result = [1, 2, 5, 7, 9, 11, 13, 15];
        counter = 0;
        for node in stailq_a.iter() {
            if node.element % 2 == 1 {
                match node.remove_after(Some(&mut stailq_a)) {
                    None => (),
                    Some(rm_node) => {
                        a.dealloc(rm_node.as_ptr() as *mut u8, size_of::<StailqNode<i32>>(), align_of::<StailqNode<i32>>());
                    }
                }
            }
        }
        for node in stailq_a.iter() {
            // rkplat::println!("counter: {}, result in node: {}, expect result: {}", counter, node.element, result[counter]);
            assert_eq!(node.element, result[counter]);
            counter += 1;
        }
        arr_len = 8;

        // test `Stailq::push_back()`
        let result = [1, 2, 5, 7, 9, 11, 13, 15, 14, 12, 10, 8];
        counter = 0;
        while counter < 4 {
            let ptr_e = alloc_type::<StailqNode<i32>>(a, StailqNode::<i32>::new((2*(7-counter)) as i32));
            let node = NonNull::new(ptr_e).expect("error: fail to get node\n");
            stailq_a.push_back(node);
            counter += 1;
        }
        counter = 0;
        for node in stailq_a.iter() {
            assert_eq!(node.element, result[counter]);
            counter += 1;
        }
        arr_len = 12;

        // test `Stailq::pop_front()` and `Stailq::is_empty()`
        counter = 0;
        while counter < arr_len {
            let node = stailq_a.pop_front().expect("error: fail to get node from pop_front()\n");
            let e = node.as_ref().element;
            assert_eq!(e, result[counter]);
            a.dealloc(node.as_ptr() as *mut u8, size_of::<StailqNode<i32>>(), align_of::<StailqNode<i32>>());
            counter += 1;
        }
        assert!(stailq_a.is_empty());
    }
    rkplat::println!("\nTest stailq0 passed!\n");
    return 0;
}
