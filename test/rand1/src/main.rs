#![no_std]
#![no_main]
#![allow(non_upper_case_globals)]

#[macro_use]
extern crate rkplat;
extern crate rkboot;
extern crate rkswrand;
use rand::prelude::*;
use rand::distributions::*;

#[no_mangle]
fn main(_args: &mut [&str])->i32 {
    let mut rand = SmallRng::from_entropy();

    println!("Uniform distribution:");
    let dis = Uniform::<i32>::from(95..=100);
    for _ in 0..10 {
        println!("{}",rand.sample(dis));
    }
    print!("\n");

    println!("Alphanumeric distribution:");
    let dis = Alphanumeric;
    for _ in 0..10 {
        println!("{}",dis.sample_string(&mut rand,16));
    }
    print!("\n");

    println!("Open01 distribution:");
    let dis = Open01;
    for _ in 0..10 {
        println!("{}",rand.sample::<f64,_>(dis));
    }
    print!("\n");

    println!("Test rand1 passed!");
    0
}
