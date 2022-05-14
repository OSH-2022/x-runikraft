#![no_std]
use core::{slice,str};
use core::mem::{align_of, size_of};
use core::ptr::addr_of;
use rkalloc::RKalloc;
use rkplat::{irq,time,bootstrap};

static mut HEAP:[u8;4096] = [0;4096];

extern "Rust" {
    fn main(args: &mut [&str])->i32;
}

#[no_mangle]
pub unsafe extern "C" fn rkplat_entry(argc: i32, argv: *mut *mut u8) -> ! {
    #[cfg(feature="alloc_buddy")]
    let a = rkalloc_buddy::RKallocBuddy::new(HEAP.as_mut_ptr(), HEAP.len());

    rkalloc::register(addr_of!(a));
    
    // 把C风格的arguments转换成Rust风格的arguments
    let args_heap = a.alloc(argc as usize*size_of::<usize>(), align_of::<usize>());
    let args = slice::from_raw_parts_mut(args_heap as *mut &str, argc as usize);
    for i in 0..(argc as usize) {
        let s = *argv.add(i);
        let mut len = 0;
        while *s.add(len) != 0 {len+=1;}
        args[i] = str::from_utf8(slice::from_raw_parts_mut(s, len)).unwrap();
    }

    rkboot_entry(&a, args);
}

unsafe fn rkboot_entry(alloc: *const dyn RKalloc, args: &mut [&str]) -> ! {
    irq::init(alloc).unwrap();
    time::init();
    let ret = main(args);
    rkplat::println!("main returned {}, halting system", ret);
    bootstrap::halt();
}
