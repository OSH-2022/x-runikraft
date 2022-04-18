/* 
build:
    cargo build --release
    rust-objcopy --strip-all build/riscv64gc-unknown-none-elf/release/dev-test -O binary build/riscv64gc-unknown-none-elf/release/dev-test.bin

run:
    qemu-system-riscv64 -machine virt -nographic -bios $RISCV_BIOS -device loader,file=build/riscv64gc-unknown-none-elf/release/dev-test.bin,addr=0x80200000 -s -S

debug:
    riscv64-unknown-elf-gdb -ex 'file build/riscv64gc-unknown-none-elf/release/dev-test.bin' -ex 'set arch riscv:rv64' -ex 'target remote localhost:1234'
    x/10i $pc: 反汇编PC后的10条指令
    b *0x80200000: 设置断点
    si: 执行一条指令
    p $r0: 查看寄存器
    c: 继续执行
    Ctrl+C: 暂停执行
*/
#![no_std]
#![no_main]

#[macro_use]
extern crate runikraft;

use rkalloc::RKalloc;
use rkalloc_empty::RKallocEmpty;
use runikraft::platform::time;

static mut HEAP_SPACE: [u8;1000] = [0;1000];

#[no_mangle]
fn main() {
    let mut alloc;
    unsafe {
        alloc = RKallocEmpty::new(HEAP_SPACE.as_mut_ptr(),1000);
    }
    println!("Hello, world!");
    let p1 = unsafe{alloc.malloc(10)};
    println!("p1={:?}",p1);
    let p2 = unsafe{alloc.malloc(5)};
    println!("p2={:?}",p2);
    println!("sleep for 10s");
    let start = time::get_ticks();
    loop {
        if (time::get_ticks() - start).as_secs()>=10 {break;}
    }
    let end = time::get_ticks();
    println!("slept for {:?}",end - start);
}
