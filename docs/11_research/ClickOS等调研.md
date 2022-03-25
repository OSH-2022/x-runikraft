# ClickOS

ClickOS 是一个基于 Xen 的高性能地虚拟化软件中间盒平台。为了达到高性能，ClickOS 对 Xen 的 I/O 子系统实现了广泛的翻修，包括对后端交换机、虚拟网络设备和后端前端驱动程序。这些更改使 ClickOS 能够显著加快中间盒运行时的网络连接。

ClickOS 虚拟机很小(5mb) ，启动速度很快(大约30毫秒) ，只需要很少的延迟(45微秒) 。ClickOS 可以将网络功能虚拟化变成现实：它在商品硬件上运行数百个中间盒，提供每秒数百万个数据包的处理速度并产生低数据包延迟。ClickOS 证明软件解决方案本身就足以显着加快虚拟机处理速度，达到剩余开销的地步与将异构中间盒处理安全地整合到同一硬件上的能力相形见绌。

ClickOS 的主要贡献是采用 Click 作为中间盒的主要编程抽象并创建量身定制的客户操作系统运行 Click 配置。这种专业化使我们能够优化中间盒的运行时间以适应他们以毫秒为单位启动的点，同时允许我们支持广泛的功能。ClickOS 实现了广泛的中间盒，包括防火墙、运营商级 NAT 和负载均衡器，并证明 ClickOS 可以每秒处理数百万个数据包，达到生产级性能。

ClickOS 可以帮助测试和部署通过将流子集引导到运行实验代码的 VM 来获得新功能；功能问题会那么只影响一小部分流量，甚至虚拟机 崩溃不会是一个大问题，因为它们可以在几毫秒内重新实例化。 

ClickOS 的架构如图：

![](pictures\clickOS_arch.jpg)

Basic ClickOS networking in Xen：

![](pictures\ClickOS_networking.png)



# MirageOS

MirageOS 是一个基于 OCaml 语言，发行在 Xen hypervisor，用于在各种云计算和移动平台构建安全、高性能网络应用程序的库操作系统。它可以将大型服务器划分为很多更小的虚拟机，使得服务器具有更强的拓展性和安全性。其代码可以在 Linux 、Mac OS 等系统中开发，然后编译成一个完全独立的、专门的内核在 Xen 或者 KVM hypervisors 和 轻量级 hypervisors 下运行。MirageOS 已经发展成为一个由近100个开放源码库组成的成熟库，实现了一系列广泛的功能，并且正开始与 Citrix XenServer 等商业产品集成。其工作原理如下，MirageOS 将 Xen hypervisors当成一个稳定的硬件平台，让我们可以专注于实施高性能协议，没必要为支持传统操作系统里面的成千上万个设备驱动程序而操心。

MirageOS 的架构如图1

![](pictures\figure1.png)

其逻辑工作流如图2。来自源代码(本地和全局库)和配置文件的精确依赖性跟踪使得已部署的内核二进制文件的完整出处被记录在不可变数据存储中，足以根据需要精确地重新编译。

![](pictures\figure2.png)

图2说明了 MirageOS 的设计。与传统的云部署周期相比，它赋予编译器更广泛的源代码依赖视角:

- 输入应用程序的所有源代码依赖项都被显式跟踪，包括实现内核功能所需的所有库。MirageOS 包括一个构建系统，该系统内部使用一个 SAT 解算器(使用 OPAM 包管理器，使用 Mancoosi 项目的解算器)从一个已发布的在线包集中搜索兼容的模块实现。由于 OCaml 的静态类型检查，在编译时将捕获接口中的任何不匹配。
- 编译器可以输出一个完整的独立内核，而不仅仅是一个 Unix 可执行文件。这些统一内核是单用途的 libOS 虚拟机，它们只执行应用程序源文件和配置文件中定义的任务，并且它们依赖于管理程序来提供资源复用和隔离。甚至启动装载程序(它必须设置虚拟内存页表并初始化语言运行时)也是作为一个简单的库编写的。每个应用程序都链接到它需要的特定库集合，并可以以特定于应用程序的方式将它们粘合在一起。
- 专门的单一内核部署在公共云上。与传统的虚拟化等价物相比，它们的攻击面要小得多，并且在引导时间、二进制大小和运行时性能方面更具资源效率。

在 MirageOS 中，OCaml 编译器接收整个内核代码的源代码，并将其链接到一个独立的本机代码对象文件。它链接到提供引导支持和垃圾回收器的最小运行时。没有抢占式线程，内核是通过一个 I/O 循环轮询 Xen 设备的事件驱动的。

图5比较了 MirageOS 和 Linux/Apache 发行版中服务的引导时间。简化的 Linux 内核和 MirageOS 的启动时间是相似的，但一旦需要初始化用户空间应用程序，效率低下的问题就会蔓延到 Linux。一旦启动，MirageOS unikernel 就可以提供通信服务。

![](pictures\figure5.png)



# IncludeOS

IncludeOS 是一个为开发基于 unikernel 的应用程序而创建 c + + API 的项目。当使用 IncludeOS 构建应用程序时，开发工具链将链接到运行应用程序所需的 IncludeOS 库的各个部分，并创建带有引导加载程序的磁盘映像。一个 IncludeOS 映像可以比运行同等程序的 Ubuntu 系统映像小几百倍。映像的启动时间为数百毫秒，这使得快速启动许多此类虚拟机映像成为可能。

当 IncludeOS 映像引导时，它通过设置内存、运行全局构造函数、注册驱动程序和中断处理程序来初始化操作系统。在 IncludeOS unikernel 中，不启用虚拟内存，应用程序和 unikernel 库使用单个地址空间。因此，没有系统调用或用户空间的概念; 所有操作系统服务都通过对库的简单函数调用来调用，并且都以特权模式运行。

IncludeOS 有如下优点：

- 性能优良，启动迅速，能在几十毫秒之内启动。
- 可移植性好，运行在虚拟机上。
- 安全性好，镜像中没有冗余代码。
- 体积小，只需很小的磁盘和内存。
- 支持在裸机上运行。
- 延迟很低。目前没有进程抢占，操作系统的行为非常静态，所以只要机器本身是可预测的，延迟也将是完全可预测的。因此，在裸机硬件上，IncludeOS可被视为低延迟，可预测的操作系统。
- 对网络的支持很好，与 Linux 相比表现出色。
- IncludeOS 系统作为一个整体进行编译和优化。在编译器和连接器阶段，优化器可以更多地了解整个系统正在做什么，并且有可能进一步优化。
- IncludeOS 中的所有 IRQ 处理程序将简单地(原子地)更新计数器，并在有时间时将进一步的处理推迟到主事件循环。这消除了对上下文切换的需要，同时也消除了与并发相关的问题，如竞争条件。通过使所有 I/O 都是异步的，CPU 保持忙碌，这样就不会发生阻塞。

IncludeOS 有如下缺点：

- IncludeOS 不实现所有 POSIX。开发人员认为，只有在需要时才会实现 POSIX 的某些部分。开发人员不太可能将完全遵守 POSIX 作为一个目标。
- 目前，IncludeOS 中没有实现阻塞调用，因为当前的事件循环模型是使用它的最佳方式。
- IncludeOS 目前还缺少可写的文件系统。



# 参考文献

- [IncludeOS: A minimal, resource efficient unikernel for cloud systems]: https://blog.acolyer.org/2016/02/22/includeos	"IncludeOS: A minimal, resource efficient unikernel for cloud systems"

- [IncludeOS: a unikernel for C++ applications]: https://lwn.net/Articles/728682/	"IncludeOS: a unikernel for C++ applications"

- [Unikernels: Rise of the Virtual Library Operating System]: https://queue.acm.org/detail.cfm?id=2566628	"Unikernels: Rise of the Virtual Library Operating System"

- [ClickOS and the Art of Network Function Virtualization]: http://cnp.neclab.eu/projects/clickos/clickos.pdf	"ClickOS and the Art of Network Function Virtualization"

- [Enabling Fast, Dynamic Network Processing with ClickOS]: http://cnp.neclab.eu/projects/clickos/clickos-workshop.pdf	"Enabling Fast, Dynamic Network Processing with ClickOS"

