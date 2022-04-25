

> by 吴骏东



## 1. RISC-V 对特权指令的支持

### 1.1 特权等级与操作系统

​		现代软件系统为了管理和配置上的安全，一般会分为不同的模块，每个模块根据需要而设置对系统和处理器资源的不同访问权限。 例如操作系统内核和用户应用程序之间的权限划分：操作系统内核具有对系统资源的高级别访问权限，而应用程序只能访问有限度的内存和寄存器。 一旦出现非法的访问请求（例如越级访问等操作），处理器将会抛出异常。这样的设计可以保证用户程序即使出现严重错误，一般也不会影响系统正常运行。

​		因此，现代操作系统（如Linux、Windows等）为了权限的区分，将特权等级状态分为为用户态和内核态。一般而言用户程序运行在用户态，而操作系统内核和设备驱动程序则运行在内核态。每个用户线程也拥有两个栈，一个是用户态栈，一个是内核态栈，分别当处于用户态和内核态时使用。操作系统的用户态和内核态对应到处理器的硬件层面上，即为不同的特权等级。

​		一般来说，用户程序一直运行在用户态中，只有当发生了一下三种事件之一时才会转到内核态：

1. 系统调用，即应用程序使用操作系统提供的接口调用内核功能；

2.  异常，当应用程序运行时出现异常时（比如最常见的缺页异常）也会切换到内核态进行处理； 

3. 外部中断，最常见的情况为当外设（如磁盘、网络适配器）完成用户请求时会向处理器发出中断，此时操作系统会暂停当前的程序运行从而转移到内核态处理这些事件。

   ​	在RISC-V中，用户态一般对应User Mode，而内核态一般对应Supervisor Mode。



### 1.2 RISC-V的特权等级

​		截至目前(2022.4)，RISC-V 共定义了四个特权级别，由高到低排列顺次为：

**机器级别（M）**：RISC-V中硬件线程可以执行的最高权限模式。在M模式下运行的 hart 对内存、I/O 和一些对于启动和配置系统来说必要的底层功能有着完全的使用权。因此它是唯一所有标准 RISC-V 处理器都必须实现的权限模式。

**超级监管者级别（H）**：为了支持虚拟机监视器。但目前该级别并没有被正式加入到文档中。

**监管者级别（S）**：为了支持现代类 Unix 操作系统，如 Linux ， FreeBSD 和 Windows 。

**用户级别（U）**：用于运行应用程序，适用于安全嵌入式系统。

<img src="assets\W1.png" alt="image-20220405231454106" style="zoom: 67%;" />

​		任何时候，一个 RISC-V 硬件线程都是运行在某个特权级上的，这个特权级由 CSR（control and status register，控制和状态寄存器）配置。且在正常情况下，线程将一直在这个特权等级下运行，除非进入trap（诸如软硬件中断、异常等）才有可能发生特权等级的转换。

​		标准的 RISC-V 为超过4000个 CSR 预留了12位的编码空间([11：0])。其中，[11：8] 位根据特权级别对 CSR 的读写权限进行编码。注意：任何试图访问不存在的 CSR 的行为均会引发指令异常，尝试写入只读寄存器也会引发指令异常。剩余的8位编码 [7 :  0] 将指定相应的寄存器地址位置。具体细节如下所示：

```
CSR [11:10]：对读写模式进行约束
	00		read/write
	01		read/write
	10		read/write
	11		read-only
CSR [9:8]：对访问最低权限进行约束
	00		Unprivileged and User-Level CSRs
	01		Supervisor-Level CSRs
	10		Hypervisor and VS CSRs
	11		Machine-Level CSRs
```

​		常用的 CSR 寄存器包括：

1. `mtvec`（Machine Trap Vector）它保存发生异常/中断时处理器需要跳转到的地址。
2. `mepc`（Machine Exception PC）它指向发生异常/中断的指令。
3. `mcause`（Machine Exception Cause）它指示发生异常/中断的原因（类型）。
4. `mie`（Machine Interrupt Enable）它指出处理器当前屏蔽了哪些中断。
5. `mip`（Machine Interrupt Pending）它列出目前正准备处理的中断（已经到来的中断）。
6. `mtval`（Machine Trap Value）它保存了陷入（trap）的附加信息：`page fault`中出错的地址、发生非法指令例外的指令本身，对于其他异常，它的值为 0。
7. `mscratch`（Machine Scratch）它暂时存放一个字大小的数据。
8. `mstatus`（Machine Status）它保存全局中断使能，以及许多其他的状态



### 1.3 RISC-V 特权指令

​		目前 RISC-V 支持的特权指令格式为：

<img src="assets\W2.png" alt="image-20220405233515809" style="zoom:67%;" />

​		以 CSR 访存指令为例。其标准格式为：

<img src="assets\W3.png" alt="image-20220405234139779" style="zoom:50%;" />

1. `csrrw`  (CSR read and write) ,这是读写操作，csr 中的值写入 rd，rs1 的值写入 csr 。
2. `csrrwi` 是csrrw的立即数扩展，rs1 寄存器保存值变为一个立即数，对 csr 的操作是一致的。
3. `csrrs `(CSR read and set), 这是读并置位操作，csr 中的值写入 rd， rs1 的值或上 csr 中的值再写入 csr 。
4. `csrrsi `是csrrs的立即数扩展，rs1 寄存器保存值变为一个立即数，对 csr 的操作是一致的。
5. `csrrc `(CSR read and clear)，这是读并清除操作，csr 中的值读入 rd，根据 rs1 的值对 csr 中的值按位清 0 再写入 csr 中。
6. `csrrci `是csrrc的立即数扩展，rs1 寄存器保存值变为一个立即数，对 csr 的操作是一致的。

​		除此以外，`ECALL`指令在Linux中用于系统调用。如下为`arch/riscv/kernel/sbi.c`中的部分代码，使用`ECALL`指令时，将异常类型写在a7 寄存器， 参数写在 a0-a5 寄存器，后面会根据异常类型的不同调用不同的异常处理函数。

```c
struct sbiret sbi_ecall(int ext, int fid, unsigned long arg0,
			unsigned long arg1, unsigned long arg2,
			unsigned long arg3, unsigned long arg4,
			unsigned long arg5)
{
	struct sbiret ret;

	register uintptr_t a0 asm ("a0") = (uintptr_t)(arg0);
	register uintptr_t a1 asm ("a1") = (uintptr_t)(arg1);
	register uintptr_t a2 asm ("a2") = (uintptr_t)(arg2);
	register uintptr_t a3 asm ("a3") = (uintptr_t)(arg3);
	register uintptr_t a4 asm ("a4") = (uintptr_t)(arg4);
	register uintptr_t a5 asm ("a5") = (uintptr_t)(arg5);
	register uintptr_t a6 asm ("a6") = (uintptr_t)(fid);
	register uintptr_t a7 asm ("a7") = (uintptr_t)(ext);
	asm volatile ("ecall"
		      : "+r" (a0), "+r" (a1)
		      : "r" (a2), "r" (a3), "r" (a4), "r" (a5), "r" (a6), "r" (a7)
		      : "memory");
	ret.error = a0;
	ret.value = a1;

	return ret;
}
```

​		例如实现一个putchar函数用于打印一个字符到系统控制台上，就如下通过ECALL来实现：

```c
void sbi_console_putchar(int ch)
{
	sbi_ecall(SBI_EXT_0_1_CONSOLE_PUTCHAR, 0, ch, 0, 0, 0, 0, 0);
}
```



## 2. KVM 对 RISC-V 架构的支持

### 2.1 KVM

​		KVM (Kernel-based Virtual Machine，基于内核的虚拟机)  ，是一种内建于 Linux 中的开源虚拟化技术。具体而言，KVM 可以将 Linux 转变为虚拟监控程序，从而使主机计算机能够运行多个隔离的虚拟环境，即虚拟客户机或虚拟机（VM）。

​		KVM 是 Linux 的一部分。其于 2006 年首次公布，并在一年后合并到主流 Linux 内核版本中。由于 KVM 属于现有的 Linux 代码，因此它能立即享受每一项新的 Linux 功能、修复和发展，而无需进行额外工程。



### 2.2 RISC-V 的虚拟化

​		目前， RISC-V 基金会对于RISC-V 的虚拟化定义了 H-Extension 规范。目前该规范还没有得到正式的批准，但其中对于 CPU 本身的虚拟化部分已经比较稳定。 RISC-V 基金会对于 RISC-V 虚拟化定义了3个实现目标：

- 支持Guest OS能够无修改地运行在Type-1, Type-2和混杂模式的虚拟机(Hypervisor)上
- 支持虚拟化嵌套
- 虚拟化带来的性能损失和实现成本不应该有太大的变化。



<img src="assets\riscv-sbi-intro2.png" alt="image-20220406095212809" style="zoom:67%;" />

​		如上图所示。为了支持虚拟化，RISC-V 规范定义了 RISC-V H-extension ，在原来的3级特权架构的基础上对原有的 Supervisor 模式进行了扩展，引入了 **Hypervisor-Extended Supervisor mode** (HS)。此时，在 Machine Mode 下运行最高优先级的、对全部资源具备操作能力的 Firmware ，虚拟机软件 Hypervisor 运行在 HS 模式，虚拟机 VM 运行在虚拟化的 Supervisor 模式，应用程序继续运行在虚拟操作系统之上，运行在 Virtualized User mode。

​		虚拟化 H 扩展定义了一个硬件状态 bit ，称作 V 状态。根据 V 状态的不同，定义和访问的 CSR 寄存器也不同。当 V 为 0 时，以 “s” 开头的 CSR 寄存器表示当前操作系统的状态， “hs” 开头的用于支持和实现虚拟化软件，而 “vs” 开头的代表运行在虚拟化技术上的系统状态。当 V 为 1 时， “s” 开头的寄存器指向了前文以 “vs” 开头的寄存器。

​		为了支持 Hypervisor ，现有的 Machine 模式下的部分寄存器需要进行增强和修改，主要体现在如下 CSR:

<img src="assets\W5.jpg" alt="img" style="zoom:67%;" />

​		同时也为 Hypervisor 模式增加了一系列的 CSR , 主要包括：

|    CSR     |                    描述                     |
| :--------: | :-----------------------------------------: |
|  hstatus   |              Hypervisor Status              |
|  hideleg   |       Hypervisor Interrupt Delegation       |
|  hedeleg   |    Hypervisor Trap/Exception Delegation     |
|    hie     |         Hypervisor Interrupt Enable         |
|   hgeie    | Hypervisor Guest External Interrupt Enable  |
| htimedelta |         Hypervisor Guest Time Delta         |
| hcounteren |          Hypervisor Counter Enable          |
|   htval    |            Hypervisor Trap Value            |
|   htinst   |         Hypervisor Trap Instruction         |
|    hip     |        Hypervisor Interrupt Pending         |
|    hvip    |    Hypervisor Virtual Interrupt Pending     |
|   hgeip    | Hypervisor Guest External Interrupt Pending |
|   hgatp    |    Hypervisor Guest Address Translation     |

​		为了实现 Supervisor 与 Hypervisor-extended supervisor 模式的切换，RSIC-V 将原来 Supervisor 模式下的 CSR 复制一份到Hypervisor，从而让每个硬件线程拥有两份 supervisor 寄存器，加快两个模式之间的切换过程。

​		由于 RISC-V 没有为不同虚拟化软件设计专门的特权模式，而是设计了统一的特权模式，这说明它对 1类、 2类虚拟化软件都有很好的支持。 RISC-V 可以通过 CSR 寄存器注入中断，因此不需要为虚拟化而特殊设计中断控制器外设。此外，RISC-V 可直接借助特殊的寄存器位支持嵌套虚拟化，而 Aarch64 要等到 v8.3 版本之后才支持这个功能。RISC-V 的时钟和核间中断可通过 SBI 软件辅助完成，而 Aarch64 需要特殊设计的计时器外设来支持虚拟化功能。

​		目前已有的虚拟化实现有 Xvisor 和 KVM 。其中 Xvisor 是1类虚拟化软件，而 KVM 属于2类。 Xvisor 起初设计是为了嵌入式硬件的虚拟化，KVM 则将 Linux 内核转变为一个虚拟化软件。



## 参考文献

1. [RISC-V特权等级与Linux内核的启动 - 知乎 (zhihu.com)](https://zhuanlan.zhihu.com/p/164394603)
2. [RISC-V基本介绍_辣椒油li的博客-CSDN博客_risc-v](https://blog.csdn.net/lijianyi0219/article/details/122634356)
3. [RISC-V 特权指令结构 - orangeQWJ - 博客园 (cnblogs.com)](https://www.cnblogs.com/orangeQWJ/p/15912780.html)
4. [闲聊RISC-V虚拟化（1）-CPU虚拟化 - 知乎 (zhihu.com)](https://zhuanlan.zhihu.com/p/408197895)
5. **The RISC-V Instruction Set Manual Volume II: Privileged Architecture**

