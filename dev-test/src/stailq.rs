#![no_std]
#![no_main]

#[macro_use]
extern crate rkplat;

use rkalloc::RKallocState;
use rkalloc_buddy::RKallocBuddy;
use runikraft::list;

static mut HEAP_SPACE: [u8;1024] = [0;1024];

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
unsafe fn main() {
    let alloc;
    alloc = RKallocBuddy::new(HEAP_SPACE.as_mut_ptr(),1024);
    let mut slist = list::STailq::<Struct>::new(&alloc);
    for i in 0..10 {
        slist.push_front(Struct::new(i)).unwrap();
        println!("after push_front {}, free_size={}",i,alloc.free_size());
        slist.push_back(Struct::new(i+1000)).unwrap();
        println!("after push_back {}, free_size={}",i+1000,alloc.free_size());
    }
    for i in slist.iter() {
        print!("{} ",i.data);
    }
    println!("");
    let mut pos = slist.head_mut();
    pos.next().unwrap();
    slist.insert_after(pos,Struct::new(15)).unwrap();
    slist.insert_after(pos,Struct::new(16)).unwrap();
    for i in slist.iter() {
        print!("{} ",i.data);
    }
    println!("");
    pos.advance(5).unwrap();
    match pos.advance(-5){
        Ok(_)=>{},
        Err(_)=>{println!("pos.advance(-5) failed");}
    }
    slist.remove_after(pos);
    slist.remove_after(pos);
    for i in slist.iter() {
        print!("{} ",i.data);
    }
    println!("");

    pos = slist.tail_mut();
    slist.insert_after(pos, Struct::new(123));
    assert_eq!(123,slist.tail().data);
    for i in slist.iter() {
        print!("{} ",i.data);
    }
    println!("");

    for i in slist.iter_mut() {
        i.data = rkplat::time::monotonic_clock().as_micros() as i32;
    }
    for i in slist.iter() {
        print!("{} ",i.data);
    }
    println!("");

    while !slist.is_empty() {
        let i = slist.pop_front().unwrap();
        println!("after pop_front {}, free_size={}",i.data,alloc.free_size());
    }
}
