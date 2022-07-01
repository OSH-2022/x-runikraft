#![no_std]
#![no_main]

use core::mem::size_of;

use rkplat::{bootstrap,thread,println};

#[repr(align(8))]
struct Arg {
    id: usize,
}

static mut THREAD_STACK: [[u8;4096];4] = [[0;4096];4];

unsafe fn thread_entry(arg: *mut u8)->! {
    let arg = (arg as *const Arg).as_ref().unwrap();
    println!("Thread #{} started.",arg.id);
    for i in 0..10 {
        println!("Thread #{}: i={}",arg.id,i);
        thread::switch(THREAD_STACK[arg.id].as_mut_ptr() as *mut thread::Context, 
            THREAD_STACK[(arg.id+1)%4].as_mut_ptr() as *mut thread::Context);
    }
    println!("Thread #{} ended.",arg.id);
    rkplat::println!("Test plat_thread_context0 passed!");
    bootstrap::halt();
}

#[no_mangle]
pub unsafe extern "C" fn rkplat_entry(_argc: i32, _argv: *mut *mut u8) -> ! {
    for i in 0..4 {
        let base = 4096 - size_of::<Arg>();
        let arg = (THREAD_STACK[i].as_mut_ptr().add(base) as *mut Arg).as_mut().unwrap();
        arg.id = i;
        thread::init(THREAD_STACK[i].as_mut_ptr() as *mut thread::Context, 
            THREAD_STACK[i].as_mut_ptr().add(base) as usize, 
            0, 
            thread_entry, 
            arg as *mut Arg as *mut u8);
    }
    thread::start(THREAD_STACK[0].as_mut_ptr() as *mut thread::Context);
}
