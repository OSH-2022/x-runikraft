// test compact_list-tailq

#![no_std]
#![no_main]
#![allow(unused_assignments)]

extern crate rkboot;
extern crate runikraft;
extern crate rkalloc;

use runikraft::compat_list::*;
use rkalloc::*;
use core::slice;
use core::mem::{size_of, align_of};
use core::ptr::NonNull;

#[no_mangle]
fn main(_args: &mut [&str])->i32 {
    let mut tailq_a = Tailq::<i32>::new();
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

        // test `Tailq::push_front()`, `TailqNode::insert_after()`, `TailqNode::insert_before()` and `Tailq::iter()`
        counter = 0;
        while counter < arr_len {
            let ptr_e = alloc_type::<TailqNode<i32>>(a, TailqNode::<i32>::new(arr[counter]));
            let mut node1 = NonNull::new(ptr_e).expect("error: fail to get node\n");
            tailq_a.push_front(node1);
            counter += 1;
            if counter < arr_len {
                let ptr_e = alloc_type::<TailqNode<i32>>(a, TailqNode::<i32>::new(arr[counter]));
                let mut node2 = NonNull::new(ptr_e).expect("error: fail to get node\n");
                node1.as_mut().insert_after(node2, Some(&mut tailq_a));
                counter += 1;
                if counter < arr_len {
                    let ptr_e = alloc_type::<TailqNode<i32>>(a, TailqNode::<i32>::new(arr[counter]));
                    let node3 = NonNull::new(ptr_e).expect("error: fail to get node\n");
                    node2.as_mut().insert_before(node3, Some(&mut tailq_a));
                    counter += 1;
                }
            }
        }
        let result = [3, 1, 2, 6, 4, 5, 9, 7, 8, 12, 10, 11, 15, 13, 14];
        counter = 0;
        for node in tailq_a.iter() {
            // rkplat::println!("counter: {}, result in node: {}, expect result: {}", counter, node.as_ref().element, result[counter]);
            assert_eq!(node.element, result[counter]);
            counter += 1;
        }

        // test `TailqNode::remove()`
        let result = [3, 1, 5, 9, 7, 11, 15, 13];
        counter = 0;
        for node in tailq_a.iter() {
            if node.element % 2 == 0 {
                node.remove(Some(&mut tailq_a));
                a.dealloc(node as *mut TailqNode<i32> as *mut u8, size_of::<TailqNode<i32>>(), align_of::<TailqNode<i32>>());
            }
        }
        for node in tailq_a.iter() {
            assert_eq!(node.element, result[counter]);
            counter += 1;
        }
        arr_len = 8;

        // test 'TailqNode::remove_after()` and `TailqNode::remove_before()`
        let result = [3, 1, 9, 13];
        counter = 0;
        for node in tailq_a.iter() {
            if node.element > 10 || node.element == 3 {
                match node.remove_before(Some(&mut tailq_a)) {
                    None => (),
                    Some(rm_node) => {
                        a.dealloc(rm_node.as_ptr() as *mut u8, size_of::<TailqNode<i32>>(), align_of::<TailqNode<i32>>());
                    }
                }
            }
            if node.element == 1 || node.element == 13 {
                match node.remove_after(Some(&mut tailq_a)) {
                    None => (),
                    Some(rm_node) => {
                        a.dealloc(rm_node.as_ptr() as *mut u8, size_of::<TailqNode<i32>>(), align_of::<TailqNode<i32>>());
                    }
                }
            }
            counter += 1;
        }
        counter = 0;
        for node in tailq_a.iter() {
            assert_eq!(node.element, result[counter]);
            counter += 1;
        }
        arr_len = 4;

        // test `Tailq::push_back()`
        let result = [3, 1, 9, 13, 12, 10, 8, 6];
        counter = 0;
        while counter < 4 {
            let ptr_e = alloc_type::<TailqNode<i32>>(a, TailqNode::<i32>::new((2*(6-counter)) as i32));
            let node = NonNull::new(ptr_e).expect("error: fail to get node\n");
            tailq_a.push_back(node);
            counter += 1;
        }
        counter = 0;
        for node in tailq_a.iter() {
            assert_eq!(node.element, result[counter]);
            counter += 1;
        }
        arr_len = 8;

        // test `Tailq::pop_front()`
        counter = 0;
        while counter < arr_len - 4 {
            let node = tailq_a.pop_front().expect("error: fail to get node from pop_front()\n");
            let e = node.as_ref().element;
            assert_eq!(e, result[counter]);
            a.dealloc(node.as_ptr() as *mut u8, size_of::<TailqNode<i32>>(), align_of::<TailqNode<i32>>());
            counter += 1;
        }
        arr_len = 4;

        // test `Tailq::pop_back()` and `Tailq::is_empty()`
        let result = [12, 10, 8, 6];
        counter = 0;
        while counter < arr_len {
            let node = tailq_a.pop_back().expect("error: fail to get node from pop_back()\n");
            let e = node.as_ref().element;
            assert_eq!(e, result[arr_len - counter - 1]);
            a.dealloc(node.as_ptr() as *mut u8, size_of::<TailqNode<i32>>(), align_of::<TailqNode<i32>>());
            counter += 1;
        }
        assert!(tailq_a.is_empty());
    }
    rkplat::println!("\nTest tailq0 passed!\n");
    return 0;
}
