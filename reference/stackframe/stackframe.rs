#[repr(C)]
#[derive(Debug)]
#[derive(Default)]
pub struct RegGenExcept {
    pub t0: usize,      //0
    pub t1: usize,      //8
    pub t2: usize,      //16
    pub t3: usize,      //24
    pub t4: usize,      //32
    pub t5: usize,      //40
    pub t6: usize,      //48
    pub a0: usize,      //56
    pub a1: usize,      //64
    pub a2: usize,      //72
    pub a3: usize,      //80
    pub a4: usize,      //88
    pub a5: usize,      //96
    pub a6: usize,      //104
    pub a7: usize,      //112
    pub ra: usize,      //120
    pub pc: usize,      //128
    pub sstatus: usize, //136
    pub sp: usize,      //144
}

#[no_mangle]
unsafe extern "C" fn __rkplat_exception_handle(cause: usize, regs: &mut RegGenExcept) {
    let (description,panic) = match cause {
        0 => ("Instruction address misaligned.",true),
        1 => ("Instruction access fault.",true),
        2 => ("Illegal instruction.",true),
        3 => ("Breakpoint.",false),
        4 => ("Load address misaligned.",true),
        5 => ("Load access fault.",true),
        6 => ("Store/AMO address misaligned.",true),
        7 => ("Store/AMO access fault.",true),
        8 => ("Environment call from U-mode.",false),
        9 => ("Environment call from S-mode.",false),
        12 => ("Instruction page fault.",true),
        13 => ("Load page fault.",true),
        15 => ("Store/AMO page fault.",true),
        _ => ("Unknown error.",true),
    };
    println!("{} (code=0x{:02x})",description,cause);
    println!("registers:");
    println!("a0 = 0x{:016x}",regs.a0);
    println!("a1 = 0x{:016x}",regs.a1);
    println!("a2 = 0x{:016x}",regs.a2);
    println!("a3 = 0x{:016x}",regs.a3);
    println!("a4 = 0x{:016x}",regs.a4);
    println!("a5 = 0x{:016x}",regs.a5);
    println!("a6 = 0x{:016x}",regs.a6);
    println!("a7 = 0x{:016x}",regs.a7);
    println!("t0 = 0x{:016x}",regs.t0);
    println!("t1 = 0x{:016x}",regs.t1);
    println!("t2 = 0x{:016x}",regs.t2);
    println!("t3 = 0x{:016x}",regs.t3);
    println!("t4 = 0x{:016x}",regs.t4);
    println!("t5 = 0x{:016x}",regs.t5);
    println!("t6 = 0x{:016x}",regs.t6);
    println!("ra = 0x{:016x}",regs.ra);
    println!("sp = 0x{:016x}",regs.sp);
    println!("pc = 0x{:016x}",regs.pc);
    if cause != 1 {
        println!("instruction = 0x{:016x}",*(regs.pc as *const usize));
    }
    if panic {
        panic!();
    }
}

fn main() {
    
}
