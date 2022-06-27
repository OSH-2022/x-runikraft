// test compact_list-list

#![no_std]
#![no_main]

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
    let mut list_a = List::<i32>::new();
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

        // test `List::push_front()`, `ListNode::insert_after()`, `ListNode::insert_before()` and `List::iter()`
        counter = 0;
        while counter < arr_len {
            let ptr_e = alloc_type::<ListNode<i32>>(a, ListNode::<i32>::new(arr[counter]));
            let mut node1 = NonNull::new(ptr_e).expect("error: fail to get node\n");
            list_a.push_front(node1);
            counter += 1;
            if counter < arr_len {
                let ptr_e = alloc_type::<ListNode<i32>>(a, ListNode::<i32>::new(arr[counter]));
                let mut node2 = NonNull::new(ptr_e).expect("error: fail to get node\n");
                node1.as_mut().insert_after(node2);
                counter += 1;
                if counter < arr_len {
                    let ptr_e = alloc_type::<ListNode<i32>>(a, ListNode::<i32>::new(arr[counter]));
                    let node3 = NonNull::new(ptr_e).expect("error: fail to get node\n");
                    node2.as_mut().insert_before(node3, Some(&mut list_a));
                    counter += 1;
                }
            }
        }
        let list_iter = list_a.iter();
        let result = [3, 1, 2, 6, 4, 5, 9, 7, 8, 12, 10, 11, 15, 13, 14];
        counter = 0;
        for node in list_iter {
            // rkplat::println!("counter: {}, result in node: {}, expect result: {}", counter, node.as_ref().element, result[counter]);
            assert_eq!(node.element, result[counter]);
            counter += 1;
        }

        // test `ListNode::remove()`
        let list_iter = list_a.iter();
        let result = [3, 1, 5, 9, 7, 11, 15, 13];
        counter = 0;
        for mut node in list_iter {
            if node.element % 2 == 0 {
                node.remove(Some(&mut list_a));
                a.dealloc(node as *mut ListNode<i32> as *mut u8, size_of::<ListNode<i32>>(), align_of::<ListNode<i32>>());
            }
        }
        let list_iter = list_a.iter();
        for node in list_iter {
            assert_eq!(node.element, result[counter]);
            counter += 1;
        }
        arr_len = 8;

        // test 'ListNode::remove_after()` and `ListNode::remove_before()`
        let list_iter = list_a.iter();
        let result = [3, 1, 9, 13];
        counter = 0;
        for mut node in list_iter {
            if node.element > 10 || node.element == 3 {
                match node.remove_before(Some(&mut list_a)) {
                    None => (),
                    Some(rm_node) => {
                        a.dealloc(rm_node.as_ptr() as *mut u8, size_of::<ListNode<i32>>(), align_of::<ListNode<i32>>());
                    }
                }
            }
            if node.element == 1 || node.element == 13 {
                match node.remove_after() {
                    None => (),
                    Some(rm_node) => {
                        a.dealloc(rm_node.as_ptr() as *mut u8, size_of::<ListNode<i32>>(), align_of::<ListNode<i32>>());
                    }
                }
            }
            counter += 1;
        }
        let list_iter = list_a.iter();
        counter = 0;
        for node in list_iter {
            assert_eq!(node.element, result[counter]);
            counter += 1;
        }
        arr_len = 4;

        // test `List::pop_front()` and `List::is_empty()`
        counter = 0;
        while counter < arr_len {
            let node = list_a.pop_front().expect("error: fail to get node from pop_front()\n");
            let e = node.as_ref().element;
            assert_eq!(e, result[counter]);
            a.dealloc(node.as_ptr() as *mut u8, size_of::<ListNode<i32>>(), align_of::<ListNode<i32>>());
            counter += 1;
        }
        assert!(list_a.is_empty());
    }
    rkplat::println!("\nTest list0 passed!\n");
    return 0;
}