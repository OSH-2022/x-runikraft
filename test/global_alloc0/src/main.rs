// test global_alloc

#![no_std]
#![no_main]
extern crate alloc;
extern crate rkplat;
extern crate rkboot;

use alloc::{boxed::Box, vec, string::String};
use core::mem::size_of_val;

#[derive(Debug)]
struct Struct {
    data: i32,
}

impl Struct {
    fn new(data: i32) -> Self{
        Struct { data }
    }
}

#[no_mangle]
fn main(_args: &mut [&str])->i32 {

    let state = rkalloc::get_default_state().unwrap();
    let old_total_size = state.total_size();
    let old_free_size = state.free_size();

    let p1 = Box::new(Struct::new(15));
    let new_total_size = state.total_size();
    let new_free_size = state.free_size();

    assert_eq!(old_total_size, new_total_size);
    assert_eq!(old_free_size, new_free_size + size_of_val(&p1));

    let old_total_size = state.total_size();
    let old_free_size = state.free_size();
    
    let mut v1 = vec![Struct::new(1),Struct::new(2),Struct::new(3),
        Struct::new(4),Struct::new(5),Struct::new(6),Struct::new(7),
        Struct::new(8),Struct::new(9),Struct::new(10)];
    let new_total_size = state.total_size();
    let new_free_size = state.free_size();

    assert_eq!(old_total_size, new_total_size);
    assert_eq!(old_free_size, new_free_size + size_of_val(&v1));

    let old_total_size = state.total_size();
    let old_free_size = state.free_size();

    v1.pop();
    let new_total_size = state.total_size();
    let new_free_size = state.free_size();

    assert_eq!(old_total_size, new_total_size);
    assert_eq!(old_free_size, new_free_size);

    let old_total_size = state.total_size();
    let old_free_size = state.free_size();

    v1.push(Struct::new(100));
    let new_total_size = state.total_size();
    let new_free_size = state.free_size();

    assert_eq!(old_total_size, new_total_size);
    assert_eq!(old_free_size, new_free_size);

    let old_total_size = state.total_size();
    let old_free_size = state.free_size();

    let mut str1 = String::from("hello, world!");
    let new_total_size = state.total_size();
    let new_free_size = state.free_size();

    assert_eq!(old_total_size, new_total_size);
    assert_eq!(old_free_size, new_free_size + size_of_val(&str1));

    str1 += "你好, 世界！";
    let new_total_size = state.total_size();
    let new_free_size = state.free_size();

    assert_eq!(old_total_size, new_total_size);
    assert_eq!(old_free_size, new_free_size + size_of_val(&str1));

    str1 = str1.replace("hello", "换出并就绪");
    let new_total_size = state.total_size();
    let new_free_size = state.free_size();

    assert_eq!(old_total_size, new_total_size);
    assert_eq!(old_free_size, new_free_size + size_of_val(&str1));

    rkplat::println!("\nTest global_alloc0 passed!\n");
    return 0;
}
