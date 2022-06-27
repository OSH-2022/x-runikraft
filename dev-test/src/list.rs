#![no_std]
#![no_main]

#[macro_use]
extern crate rkplat;
extern crate rkboot;

use rkalloc::RKallocState;
use rkalloc_buddy::RKallocBuddy;
use runikraft::list;
use runikraft::align_as;

static mut HEAP_SPACE: align_as::A4096<[u8;1024]> = align_as::A4096::new([0;1024]);

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
unsafe fn main(_args: &mut [&str])->i32 {
    let alloc;
    alloc = RKallocBuddy::new(HEAP_SPACE.data.as_mut_ptr(),1024);
    let mut dlist = list::List::<Struct>::new(&alloc);
    for i in 0..10 {
        dlist.push_front(Struct::new(i)).unwrap();
        println!("after push_front {}, free_size={}",i,alloc.free_size());
    }
    for i in dlist.iter() {
        print!("{} ",i.data);
    }
    println!("");
    let mut pos = dlist.head_mut();
    pos.next().unwrap();
    dlist.insert_after(pos,Struct::new(15)).unwrap();
    dlist.insert_before(pos,Struct::new(16)).unwrap();
    pos.prev().unwrap();
    pos.prev().unwrap();
    assert!(pos.is_head());
    dlist.insert_before(pos, Struct::new(-100)).unwrap();
    for i in dlist.iter() {
        print!("{} ",i.data);
    }
    println!("");
    pos.advance(5).unwrap();
    dlist.remove_after(pos);
    dlist.remove_after(pos);
    pos.advance(-5).unwrap();
    (_, pos) = dlist.remove(pos);
    dlist.remove_before(pos);
    assert!(pos.is_head());
    for i in dlist.iter() {
        print!("{} ",i.data);
    }
    println!("");

    for i in dlist.iter_mut() {
        i.data = rkplat::time::monotonic_clock().as_micros() as i32;
    }
    for i in dlist.iter() {
        print!("{} ",i.data);
    }
    println!("");
    {
        let mut i = dlist.iter();
        for _ in 1..dlist.len() {i.next();}
        while let Some(i) = i.prev() {
            print!("{} ",i.data);
        }
    }
    println!("");
    while !dlist.is_empty() {
        let i = dlist.pop_front().unwrap();
        println!("after pop_front {}, free_size={}",i.data,alloc.free_size());
    }
    0
}
