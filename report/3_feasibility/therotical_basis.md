# 理论依据

## Rust Language

A language empowering everyone to build reliable and efficient software.

### Advantages

- Performance: Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.

- Reliability: Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — enabling you to eliminate many classes of bugs at compile-time.

- Productivity: Rust has great documentation, a friendly compiler with useful error messages, and top-notch tooling — an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspections, an auto-formatter, and more.

### Applications

用Rust改写项目已经成为一种解决问题的有效手段，如zalando公司从Scala转向Rust的成功故事。[^3]

## RISC-V Architecture

RISC-V is a free and open ISA enabling a new era of processor innovation through open standard collaboration.

The RISC-V ISA delivers a new level of free, extensible software and hardware freedom on architecture, paving the way for the next 50 years of computing design and innovation.[^0]

### Advantages

1. RISC（精简指令集计算）架构具有一组指令，因此高级语言编译器可以生成更有效的代码。
2. 由于其简单性，它允许自由使用微处理器上的空间。
3. 许多RISC处理器使用寄存器来传递参数和保存局部变量。
4. RISC函数仅使用几个参数，而RISC处理器无法使用调用指令，因此，使用易于流水线化的固定长度指令。
5. 操作速度可以最大化，执行时间可以最小化。
6. 所需的指令格式数量很少，所需的指令数量和寻址方式也很少。[^4]

### Applications

目前基于RISC-V架构的开源处理器有很多，既有标量处理器Rocket，也有超标量处理器BOOM，还有面向嵌入式领域的Z-scale、PicoRV32等。[^2]

## QEMU

QEMU is a generic and open source machine emulator and virtualizer.

When used as a machine emulator, QEMU can run OSes and programs made for one machine (e.g. an ARM board) on a different machine (e.g. your own PC). By using dynamic translation, it achieves very good performance.

When used as a virtualizer, QEMU achieves near native performance by executing the guest code directly on the host CPU. QEMU supports virtualization when executing under the Xen hypervisor or using the KVM kernel module in Linux. When using KVM, QEMU can virtualize x86, server and embedded PowerPC, 64-bit POWER, S390, 32-bit and 64-bit ARM, and MIPS guests.[^1]

### Advantages

- Full-system emulation: Run operating systems for any machine, on any supported architecture.

- User-mode emulation: Run programs for another Linux/BSD target, on any supported architecture.

- Virtualization: Run KVM and Xen virtual machines with near native performance.[^1]

[^0]:[About RISC-V - RISC-V International (riscv.org)](https://riscv.org/about/)
[^1]:[QEMU](https://wiki.qemu.org/Main_Page)
[^2]:[RISC-V开源指令集架构简介与应用前景分析 | ScenSmart一站式智能制造平台|OEM|ODM|行业方案](http://www.scensmart.com/news/the-introduction-of-risc-v-and-application-prospect-analysis/)
[^3]:[故事：Rust在企业领域的应用 - 知乎 (zhihu.com)](https://zhuanlan.zhihu.com/p/61410107)
[^4]:[什么是RISC架构？RISC架构的优点与缺点 (enroo.com)](http://www.enroo.com/support/category1/dpjrmzs/78378500.html)

