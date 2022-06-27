#![no_std]
#![no_main]

extern crate rkboot;
use rkplat::{println};
use rkplat::console::{cink,coutk};
use core::str;

#[no_mangle]
fn main(_args: &mut [&str])->i32 {
    println!("uart test");
    let mut buffer = [0u8;128];
    loop {
        if let Some(cnt) = cink(&mut buffer) {
            let s = str::from_utf8(&buffer[0..cnt]).unwrap();
            if s == "\x03" {break;}
            coutk(s.as_bytes());
        }
    }
    println!("\n(Exit)");
    0
}
