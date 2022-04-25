# Unikernel的安全性问题

根据[NCC Group](https://research.nccgroup.com/wp-content/uploads/2020/07/ncc_group-assessing_unikernel_security.pdf)，虽然 unikernel 相比容器体积更小、隔离性更好，但是由于不存在内核态-用户态隔离，且缺乏W^X、stack canary等安全特性，unikernel 其实比传统的容器更不安全。也就是说，攻击者可以利用 unikernel 上的程序的漏洞，控制 unikernel 所在的虚拟机，进而获取本不应该拥有的权限。以 unikernel 的传统应用领域云计算为例，如果某个 unikernel 负责处理用户的敏感数据——它通过网络获取用户的数据，然后将计算结果通过网络发回，则它一定拥有读用户数据的权限。那么，一旦这个 unikernel 存在安全漏洞，攻击者虽然不能控制 unikernel 所在的宿主机，但足够窃取用户的数据。所以，我们不能片面地把安全性与隔离性等同。而且我们不能片面地认为使用 Rust 这样的安全的程序设计语言就能保证安全，因为完整的 unikernel 上不只包含安全的系统代码（或者说库代码），还包含可能不安全的用户代码，而后者可以导致整个系统不安全。因此，要实现安全的 unikernel，不能仅仅依靠安全的程序设计语言，而需要额外的安全特性。

NCC Group提到的安全特性：

- 地址空间布局随机化 （ASLR）：将数据、函数的位置随机放在内存段中，这样将无法通过跳转到特定地址完成攻击。
- 分页保护
  - W^X政策：一段内存空间不能同时拥有执行权和写入权；
  - 内部数据加固：防止程序读取与程序本身无关的数据，比如全局偏移表；
  - 保护页：在不同数据段之间（比如.text段与.data）放置不可读写的页面；
  - 空页面漏洞：`malloc`函数可能返回`nullptr`，而在 unikernel 上，`0x0`可能是有效地址，访问0不会引发segmentation fault。
- 栈保护标志（stack canary，[典故](https://en.wikipedia.org/wiki/Animal_sentinel#Historical_examples)：金丝雀曾经被用来检查煤矿的有毒气体）：在栈的返回地址后加上一个随机的整型变量（canary），执行`ret`前检查 canary 是否被修改。
- 堆加固：堆的结构通常是双向链表，链表的元数据（如指针）通常与数据块相邻，堆上的溢出可以改变这些数据块，导致内存的分配、释放算法出错，通常的加固方法是为元数据增加校验值。
- 熵和随机数生成器：通常的 unikernel 缺乏足够产生密码学安全的伪随机数的硬件熵，这导致 unikernel 上生成的伪随机数的质量差，解决方法有使用 CPU 的专用随机数指令（如 x86 的 `rdrand`）
- 标准库加固
  - `printf` 的 `%n` 格式符：它将已经输出的字符的数量写入对应的参数指向的地址（没错，它不输出任何内容，而是写入内容），攻击者可以利用自定义的格式串和 `%n` 实现攻击，所以不应该支持它；
  - 自定义格式符：它可能可以扩大攻击面；
  - `_FORTIFY_SOURCE` 宏：定义它会导致一些函数执行轻量级的缓冲区溢出检查。

NCC Group只测试了 rumprun 和 includeOS 两个 unikernels，并发现它们几乎没有实现任何安全特性。

尽管 Unikraft 使用 C 语言实现，但它支持（或计划支持）以下安全特性：

| Security feature                                             | Status           | Targets                      |
| :----------------------------------------------------------- | :--------------- | :--------------------------- |
| [Stack Smashing Protection (SP)](https://github.com/unikraft/unikraft/tree/staging/lib/uksp) | Upstream         | `ARCH_ARM_64 || ARCH_X86_64` |
| [Undefined Behavior Sanitization (UBSAN)](https://github.com/unikraft/unikraft/tree/staging/lib/ubsan) | Upstream         | any                          |
| [Rust internal libraries in Unikraft](https://github.com/unikraft/unikraft/tree/staging/lib/ukrust) | Upstream         | `ARCH_X86_64`                |
| [ARM Pointer authentication (PAuth)](https://unikraft.org/#) | Under review     | `ARCH_ARM_64 || ARCH_ARM_32` |
| [ARM Branch Target Identification (BTI)](https://github.com/unikraft/unikraft/pull/421) | Under review     | `ARCH_ARM_64`                |
| [Kernel Address Sanitizer (KASAN)](https://github.com/unikraft/unikraft/pull/191) | Under review     | `PLAT_KVM && ARCH_X86_64`    |
| [Position Independent Executables (PIE)](https://github.com/unikraft/unikraft/pull/239) | Under review     | `PLAT_KVM && ARCH_X86_64`    |
| [True Random Number Generator](https://unikraft.org/#)       | Under review     | `ARCH_X86_64`                |
| ARM Memory Tagging Extension (MTE)                           | Work-in-progress | ARM                          |
| Intel Control-flow Enforcement Technology (CET)              | Planned          | `ARCH_X86_64`                |
| Shadow stack                                                 | Planned          | any                          |
| `FORTIFY_SOURCE`                                             | Planned          | any                          |
| ARM Speculation Barrier (SB)                                 | Planned          | `ARCH_ARM_64`                |
| Kernel Page Table Isolation (KPTI)                           | N/A              | N/A                          |
| Supervisor Mode Access Prevention (SMAP)                     | N/A              | N/A                          |
| Privileged Access Never (PAN)                                | N/A              | N/A                          |
