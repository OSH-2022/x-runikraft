/* 
build:
    cargo build --release
    rust-objcopy --strip-all build/riscv64gc-unknown-none-elf/release/dev-test -O binary build/riscv64gc-unknown-none-elf/release/dev-test.bin

run:
    qemu-system-riscv64 -machine virt -nographic -bios $RISCV_BIOS -device loader,file=build/riscv64gc-unknown-none-elf/release/dev-test.bin,addr=0x80200000 -s -S

debug:
    riscv64-unknown-elf-gdb -ex 'file build/riscv64gc-unknown-none-elf/release/dev-test' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'
    x/10i $pc: 反汇编PC后的10条指令
    b *0x80200000: 设置断点
    si: 执行一条指令
    p $r0: 查看寄存器
    c: 继续执行
    Ctrl+C: 暂停执行
*/
#![no_std]
#![no_main]

use runikraft as rk;

use rkalloc::RKalloc;
use rkalloc_buddy::RKallocBuddy;
// use rk::plat::time;

static mut HEAP_SPACE: [u8;1024] = [0;1024];

#[no_mangle]
fn main() {
    let alloc;
    unsafe {
        alloc = RKallocBuddy::new(HEAP_SPACE.as_mut_ptr(),1024);
    }
    rk::println!("Hello, world!");
    rk::println!("base = {:?}",unsafe{HEAP_SPACE.as_mut_ptr()});
    let mut ptr = [0 as *mut u8;64];
    for i in 0..32 {
        ptr[i*2] = unsafe {alloc.alloc(16, 16)};
        ptr[i*2+1] = unsafe {alloc.alloc(32, 16)};
        rk::println!("p{}={:?}",i*2,ptr[i*2]);
        rk::println!("p{}={:?}",i*2+1,ptr[i*2+1]);
        unsafe {alloc.dealloc(ptr[i*2+1], 32, 16);}
    }
    for i in 0..32 {
        unsafe {alloc.dealloc(ptr[i*2], 16, 16);}
    }
//     rk::println!("sleep for 10s");
//     let start = time::get_ticks();
//     loop {
//         if (time::get_ticks() - start).as_secs()>=10 {break;}
//     }
//     let end = time::get_ticks();
//     rk::println!("slept for {:?}",end - start);
// }
}
