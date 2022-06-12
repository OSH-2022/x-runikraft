use super::reg;
use super::bootstrap;

//Rust 的缺陷：需要巨大的栈空间：一个println!在release模式下需要96bytes的栈空间，在debug模式下需要192bytes的栈空间
//完整的函数在release模式下需要192bytes的栈空间，而在debug模式下需要2032bytes的栈空间
#[no_mangle]
unsafe extern "C" fn __rkplat_exception_handle(cause: usize, regs: &mut reg::RegGenExcept) {
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
        bootstrap::crash();
    }
}
