#include <fmt/core.h>
#include <utility>

struct RegGenExcept {
    size_t t0;      //0
    size_t t1;      //8
    size_t t2;      //16
    size_t t3;      //24
    size_t t4;      //32
    size_t t5;      //40
    size_t t6;      //48
    size_t a0;      //56
    size_t a1;      //64
    size_t a2;      //72
    size_t a3;      //80
    size_t a4;      //88
    size_t a5;      //96
    size_t a6;      //104
    size_t a7;      //112
    size_t ra;      //120
    size_t pc;      //128
    size_t sstatus; //136
    size_t sp;      //144
};

extern "C" {
    void __rkplat_exception_handle(size_t cause, RegGenExcept &regs);
}

void __rkplat_exception_handle(size_t cause, RegGenExcept &regs) {
    using std::make_pair;
    auto [description,panic] = [&]{ switch (cause) {
        case 0: return make_pair ("Instruction address misaligned.",true);
        case 1 : return make_pair("Instruction access fault.",true);
        case 2 : return make_pair("Illegal instruction.",true);
        case 3 : return make_pair("Breakpoint.",false);
        case 4 : return make_pair("Load address misaligned.",true);
        case 5 : return make_pair("Load access fault.",true);
        case 6 : return make_pair("Store/AMO address misaligned.",true);
        case 7 : return make_pair("Store/AMO access fault.",true);
        case 8 : return make_pair("Environment call from U-mode.",false);
        case 9 : return make_pair("Environment call from S-mode.",false);
        case 12: return make_pair("Instruction page fault.",true);
        case 13: return make_pair("Load page fault.",true);
        case 15: return make_pair("Store/AMO page fault.",true);
        default: return make_pair("Unknown error.",true);
    }}();
    fmt::print("{} (code=0x{:02x})\n",description,cause);
    fmt::print("registers:\n");
    fmt::print("a0 = 0x{:016x}\n",regs.a0);
    fmt::print("a1 = 0x{:016x}\n",regs.a1);
    fmt::print("a2 = 0x{:016x}\n",regs.a2);
    fmt::print("a3 = 0x{:016x}\n",regs.a3);
    fmt::print("a4 = 0x{:016x}\n",regs.a4);
    fmt::print("a5 = 0x{:016x}\n",regs.a5);
    fmt::print("a6 = 0x{:016x}\n",regs.a6);
    fmt::print("a7 = 0x{:016x}\n",regs.a7);
    fmt::print("t0 = 0x{:016x}\n",regs.t0);
    fmt::print("t1 = 0x{:016x}\n",regs.t1);
    fmt::print("t2 = 0x{:016x}\n",regs.t2);
    fmt::print("t3 = 0x{:016x}\n",regs.t3);
    fmt::print("t4 = 0x{:016x}\n",regs.t4);
    fmt::print("t5 = 0x{:016x}\n",regs.t5);
    fmt::print("t6 = 0x{:016x}\n",regs.t6);
    fmt::print("ra = 0x{:016x}\n",regs.ra);
    fmt::print("sp = 0x{:016x}\n",regs.sp);
    fmt::print("pc = 0x{:016x}\n",regs.pc);
    if(cause != 1) 
        fmt::print("instruction = 0x{:016x}\n",*(reinterpret_cast<size_t*>(regs.pc)));
    if(panic)
        abort();
} 

int main() {
    
}
