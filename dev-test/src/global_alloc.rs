#![no_std]
#![no_main]

#[macro_use]
extern crate rkplat;
extern crate rkboot;
extern crate alloc;
use alloc::{boxed::Box, vec, string::String};

#[derive(Debug)]
struct Struct {
    data: i32,
}

impl Struct {
    fn new(data: i32) -> Self{
        Struct { data }
    }
}

impl Drop for Struct {
    fn drop(&mut self) {
        println!("\x1b[38;2;240;0;0mdrop {}\x1b[0m",self.data);
    }
}

#[no_mangle]
fn main(_args: &mut [&str])->i32 {
    let state = rkalloc::get_default_state().unwrap();
    println!("at beginning \x1b[38;2;0;240;0mtotal size={}, free size={}\x1b[0m",state.total_size(),state.free_size());
    let mut p1 = Box::new(Struct::new(15));
    println!("after box::new \x1b[38;2;0;240;0mtotal size={}, free size={}\x1b[0m",state.total_size(),state.free_size());
    p1.data = -6;
    let mut v1 = vec![Struct::new(1),Struct::new(2),Struct::new(3),
        Struct::new(4),Struct::new(5),Struct::new(6),Struct::new(7),
        Struct::new(8),Struct::new(9),Struct::new(10)];
    println!("after vec![]: \x1b[38;2;0;240;0mtotal size={}, free size={}\x1b[0m",state.total_size(),state.free_size());
    for i in v1.iter_mut() {
        print!("{} ",i.data);
        i.data -= 1;
    }
    println!("\nvec={:?}",&v1);
    v1.pop();
    println!("after pop(): \x1b[38;2;0;240;0mtotal size={}, free size={}\x1b[0m",state.total_size(),state.free_size());
    println!("\nvec={:?}",&v1);
    v1.push(Struct::new(100));
    println!("after push(): \x1b[38;2;0;240;0mtotal size={}, free size={}\x1b[0m",state.total_size(),state.free_size());
    println!("\nvec={:?}",&v1);
    let mut str1 = String::from("hello, world!");
    println!("after String::from(): \x1b[38;2;0;240;0mtotal size={}, free size={}\x1b[0m",state.total_size(),state.free_size());
    str1 += "你好, 世界！";
    println!("after +=: \x1b[38;2;0;240;0mtotal size={}, free size={}\x1b[0m",state.total_size(),state.free_size());
    println!("str1={}",&str1);
    str1 = str1.replace("hello", "换出并就绪");
    println!("after replace: \x1b[38;2;0;240;0mtotal size={}, free size={}\x1b[0m",state.total_size(),state.free_size());
    println!("str1={}",&str1);
    0
}
