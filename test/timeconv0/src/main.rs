#![no_std]
#![no_main]

extern crate rkboot;
#[macro_use]
extern crate rkplat;
use rktimeconv::TimePoint;
use core::time::Duration;

#[no_mangle]
fn main(_args: &mut [&str])->i32 {
    let tp1 = TimePoint::from_unix_time(Duration::new(1656733945, 1000));
    println!("{}, {}, {}, {}, {}, {}", tp1.year(), tp1.month(), tp1.day(), tp1.hour(), tp1.min(), tp1.second());
    println!("{}, {}, {}", tp1.nanosec(), tp1.day_in_week(), tp1.day_in_year());
    assert_eq!(tp1.year(),2022);
    assert_eq!(tp1.month(),7-1);
    assert_eq!(tp1.day(),2);
    assert_eq!(tp1.hour(), 3);
    assert_eq!(tp1.min(), 52);
    assert_eq!(tp1.second(), 25);
    assert_eq!(tp1.nanosec(), 1000);
    assert_eq!(tp1.day_in_week(), 6);
    assert_eq!(tp1.day_in_year(), 183-1);
    let tp2 = TimePoint::from_unix_time(Duration::new(1645713945, 50000));
    println!("{}, {}, {}, {}, {}, {}", tp2.year(), tp2.month(), tp2.day(), tp2.hour(), tp2.min(), tp2.second());
    println!("{}, {}, {}", tp2.nanosec(), tp2.day_in_week(), tp2.day_in_year());
    assert_eq!(tp2.year(),2022);
    assert_eq!(tp2.month(),2-1);
    assert_eq!(tp2.day(),24);
    assert_eq!(tp2.hour(), 14);
    assert_eq!(tp2.min(), 45);
    assert_eq!(tp2.second(), 45);
    assert_eq!(tp2.nanosec(), 50000);
    assert_eq!(tp2.day_in_week(), 4);
    assert_eq!(tp2.day_in_year(), 55-1);
    let tp3 = TimePoint::from_unix_time(Duration::new(1545713945, 123));
    println!("{}, {}, {}, {}, {}, {}", tp3.year(), tp3.month(), tp3.day(), tp3.hour(), tp3.min(), tp3.second());
    println!("{}, {}, {}", tp3.nanosec(), tp3.day_in_week(), tp3.day_in_year());
    assert_eq!(tp3.year(),2018);
    assert_eq!(tp3.month(),12-1);
    assert_eq!(tp3.day(),25);
    assert_eq!(tp3.hour(), 4);
    assert_eq!(tp3.min(), 59);
    assert_eq!(tp3.second(), 5);
    assert_eq!(tp3.nanosec(), 123);
    assert_eq!(tp3.day_in_week(), 2);
    assert_eq!(tp3.day_in_year(), 359-1);
    rkplat::println!("Test timeconv0 passed!");
    0
}
