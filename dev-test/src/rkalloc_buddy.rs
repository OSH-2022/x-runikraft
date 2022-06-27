#![no_std]
#![no_main]

#[macro_use]
extern crate rkplat;
extern crate rkboot;

use rkalloc::{RKalloc, RKallocExt, RKallocState};
use rkalloc_buddy::RKallocBuddy;
use runikraft::align_as;

static mut HEAP_SPACE: align_as::A4096<[u8;1024]> = align_as::A4096::new([0;1024]);

#[no_mangle]
unsafe fn main(_args: &mut [&str])->i32 {
    let alloc;
    alloc = RKallocBuddy::new(HEAP_SPACE.data.as_mut_ptr(),1024);
    println!("base = {:?}",HEAP_SPACE.data.as_mut_ptr());
    println!("total size={}, free size={}",alloc.total_size(),alloc.free_size());
    println!("\x1b[38;2;0;240;0m{}: \x1b[38;2;240;0;0m{:?}\x1b[0m","After new",alloc);
    let mut ptr = [0 as *mut u8;64];
    for i in 0..32 {
        ptr[i*2] = alloc.alloc(16, 16);
        println!("\x1b[38;2;0;240;0m{}: \x1b[38;2;240;0;0m{:?}\x1b[0m","After alloc 16",alloc);
        ptr[i*2+1] = alloc.alloc(32, 16);
        println!("\x1b[38;2;0;240;0m{}: \x1b[38;2;240;0;0m{:?}\x1b[0m","After alloc 32",alloc);
        println!("p{}={:?}",i*2,ptr[i*2]);
        println!("p{}={:?}",i*2+1,ptr[i*2+1]);
        println!("free size={}",alloc.free_size());
        alloc.dealloc_ext(ptr[i*2+1]);
        println!("\x1b[38;2;0;240;0m{}: \x1b[38;2;240;0;0m{:?}\x1b[0m","After dealloc 32",alloc);
    }
    for i in 0..32 {
        alloc.dealloc_ext(ptr[i*2]);
        println!("\x1b[38;2;0;240;0m{}: \x1b[38;2;240;0;0m{:?}\x1b[0m","After dealloc 16",alloc);
        println!("free size={}",alloc.free_size());
    }
    ptr[0]=alloc.alloc(512, 1);
    println!("ptr[0]={:?}, free size={}",ptr[0],alloc.free_size());
    println!("\x1b[38;2;0;240;0m{}: \x1b[38;2;240;0;0m{:?}\x1b[0m","After alloc 512",alloc);

    ptr[1]=alloc.alloc(128, 1);
    println!("ptr[1]={:?}, free size={}",ptr[1],alloc.free_size());
    println!("\x1b[38;2;0;240;0m{}: \x1b[38;2;240;0;0m{:?}\x1b[0m","After alloc 128",alloc);

    alloc.dealloc_ext(ptr[0]);
    println!("free size={}",alloc.free_size());
    println!("\x1b[38;2;0;240;0m{}: \x1b[38;2;240;0;0m{:?}\x1b[0m","After dealloc 512",alloc);

    ptr[0]=alloc.alloc(256, 1);
    println!("ptr[0]={:?}, free size={}",ptr[0],alloc.free_size());
    println!("\x1b[38;2;0;240;0m{}: \x1b[38;2;240;0;0m{:?}\x1b[0m","After alloc 256",alloc);

    0

//     rk::println!("sleep for 10s");
//     let start = time::get_ticks();
//     loop {
//         if (time::get_ticks() - start).as_secs()>=10 {break;}
//     }
//     let end = time::get_ticks();
//     rk::println!("slept for {:?}",end - start);
// }
}
