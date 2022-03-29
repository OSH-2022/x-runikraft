

> 作者：				吴骏东
>
> 最后编辑时间:	2022.3.25



### 2021年

#### x-unipanic 小组

##### 项目简介

​		该项目旨在已有项目的基础上，小组希望在保持 Unikernel 现有优势（高效、安全、轻量）的前提下，改善 Unikernel 对二进制程序的支持，做出可以即时打包、分发的 Unikernel。目前致力于提供二进制兼容性的 Unikernel 项目 **HermiTux** 仍有较大改进空间，因此该小组将改善 **HermiTux** 二进制兼容性作为立项目标。

关键词：UniKernel, 进程调度

参考项目：[HermiTux](https://github.com/ssrg-vt/hermitux)



##### 项目可行性分析

- 目前的 Unikernel 实现均要求对应用的重构，在实际应用中无法获取程序源码、程序依赖未被支持等问题非常常见
- 将应用打包为 Unikernel 要求大量的专业知识，步骤繁琐
- HermiTux设计了一个二进制分析工具，能够扫描一个可执行文件，并检测该程序可以进行的各种系统调用
- HermiTux基于[hermitcore](https://github.com/hermitcore/rusty-hermit)这一Unikernel架构做了二进制支持，并重写syscall以保证性能。
- HermiTux的内核中实现了一个基本的RAM文件系统——MiniFS，从而在这方面消除了对主机的依赖。



##### 项目困难点分析

- 如果无法获得程序源代码，重新编译和链接将无从进行，也就不可能打包到 Unikernel。对二进制文件的逆向往往会因为编译过程中的剥离和混淆难以进行，因此用unikernel层进行拆解和重新链接是不合适的。
- 让 Unikernel 支持某种语言十分困难，Unikernel 通常只支持一小部分的内核特性和软件库。如果语言用到了不支持的内容，就需要重写应用，很多情况下这意味着应用完全不可能被移植。
- Unikernel 使用复杂的构建工具，将一些传统应用的大型构建基础架构（大量的 Makefile、autotools/cmake 环境）加入 Unikernel 工具链是十分麻烦。并且，unikernel还缺乏一些开发工具，如调试器（debugger）和分析工具（profiler）。



##### 项目成果分析

​		该小组主要参照了KylinX和Hermitux这两个项目。KylinX项目提供实现fork的思路；Hermitux主要实现Unikernel的二进制支持，可以在Hermitux的源码上进行改动。主要的成果有：

1. 支持fork。参照了KylinX实现fork的方式，通过复制hypervisor启动新的虚拟机作为子进程。但是这样实现的性能较低，增大了系统负担。

2. 优化重写syscall。修改了syscall打包的判断方式，将向后打包扩展成向前打包，从而100%重写了syscall函数。保持Unikernel因没有系统调用而具有的优良运行速度。




#### x-KATA-Unikernel 小组

##### 项目简介

​		该项目利用 Unikernel 得天独厚的轻量和攻击面小的特性，结合虚拟化技术，为FaaS（Function As A Service）场景下的云服务提出一种解决方案：从客户端提交代码，到云平台进行 Serverless 运算。采用 KVM 的虚拟机接口，在虚拟化环境中以 Unikernel 减少资源开销，达到空间的高效利用和速度的极限提升。

关键词：UniKernel, 虚拟化, 云计算

参考项目：[Kata](https://github.com/kata-containers/kata-containers)、[gVisor](https://github.com/google/gvisor/blob/master/README.md)、[Firecracker文档](https://firecracker-microvm.github.io/)



##### 项目可行性分析

- Firecracker 是在 rust 众多 crates 基础上实现的 VMM。它拥有非常有限的设备模型，提供轻量级的服务并且暴露的攻击面极小，在 FaaS 场景下有极大的应用空间。但其本质上还是传统的虚拟机架构，不可避免地带来多层嵌套的性能损耗。
- Google 提出的 gVisor 解决方案， 在容器的后端将所有的系统调用截断，凭借 gVisor 中用户程序来实现系统调用的API。 gVisor 极其轻量，隔离性相对不足。此外，其也面临着过多系统调用时无法忍受的上下文转换问题。并且，gVisor 采用了带有 GC 的 Go 语言编写，也有比较大的性能开销。
- Unikernel 的缺点可以被 kata Container易于分发的优点改善，同时纳入 kubernetes 生态，使得 Unikernel 的应用更加广泛。
- KVM 是采用硬件虚拟化技术的全虚拟化解决方案。其优势有：依赖 Linux 内核的内存管理、存储和客户机镜像格式多样、支持实时迁移与状态保存、支持高性能I/O接口、性能极强等。



##### 项目困难点分析

- Unikernel 的迁移问题。虽然 Unikernel 的概念被提出很久，市面上也涌现很多 Unikernel 的具体实现，但要找到易于适配 KVM，并且功能齐全的 core，是一件比较困难的事情。[项目使用了 Nanos解决]
- 虚拟机对象问题。缺少统一的方式定义虚拟机的各种可管理对象。[项目使用了 libvirt 相关工具解决]
- 人机交互问题。与客户端的交互需要将虚拟机内部的结果重定向到主机，此过程中对结果的保护和加密是十分重要的。但这需要较多的知识积累。



##### 项目成果分析

​		该小组针对当前常用的两种解决方案 Firecracker microVM 和 gVisor 进行了改造与借鉴，利用 Firecracker 基于 KVM 和 virIO 的架构获得优异的封装和性能提升，同时希望借鉴 gVisor 系统调用截断的方式，使其与 Unikernel 进行交互，取代 gVisor 中 sentry+gofer 的类内核架构，从而达到轻量高效的目的。相关成果如下：

1. 使用了支持多种语言环境的 Nanos 内核，以 KVM 作为 Unikernel 的载体。
2. 分离 Nanos 的编译编排工具 ops 中的 build 模块，对虚拟机进行硬件加速。
3. 封装 libvirt API ，从而可以更加方便地创建与管理虚拟机。
4. 使用 virt-viewer 工具实现了虚拟机可视化。

​		改造后的 Unikernel 在算法性能上相较于传统 Linux 提升了约40%。后续还可以将 Nanos 进一步与 Firecracker 结合，microVM 与 Unikernel 的结合可以将性能发挥到极限。



### 2020年

#### x-orz小组

##### 项目简介

​		该项目将一般网络程序中的任务看作各种（并发的）基本服务的组合，抽象出一些常用的服务并让每个 Unikernel 与一个服务相对应，构成Unikernel实例的集群。通过合理地编排调度 Unikernel 集群，将各种并发的服务组合起来，处理任务请求，从而充分利用多核/多CPU资源，提高系统性能，同时又不破坏 Unikernel 原有的轻量、安全的特性。

关键词：[Unikernel](http://unikernel.org/), 云计算, 高性能计算

参考项目：[Firecracker](https://github.com/firecracker-microvm/firecracker) 、 x-Doudou 



##### 项目可行性分析

- Unikernel 省去了上下文切换、进程管理、资源竞争等工作带来的开销，但这样无法充分利用多核尤其是多 CPU 的资源。单个 Unikernel 进程通常仅使用一个核。支持多核的 Unikernel 往往需要引入OS中有关进程管理、资源分配的复杂模块，这样便会破坏 Unikernel 的高精简度。
- 小规模的多进程任务可以将其修改为多线程从而装入同一个 Unikernel 。但大规模任务只能启动更多的 Unikernel 实例，从而造成相同模块的重复使用。
- 服务的拆分提高了系统容错性。因为一个 Unikernel 实例相当于一个虚拟服务器，它的崩溃不会影响整个任务的执行，调度系统只需要再创建/调度另一个提供同样服务的 Unikernel 即可。
- Firecracker 是一个由AWS开发的轻量级 Hypervisor，旨在加速他们的Serverless服务。其仅实现了五种必要的I/O设备：virtio-net、virtio-block、virtio-vsock、串口、键盘，而且它的的启动过程也更为简单，省去了实模式加载等步骤，有着显著的性能提升。



##### 项目困难点分析

- 相关 OSv 内核的管理工具大部分是为虚拟机或容器开发的，不容易保留原本 OSv+Firecracker 方案的优势（如冷启动时间）。



##### 项目成果分析

​		该小组参考研究了工业控制系统的结构。其大体的工作流程是在 Interface 部分利用传感器等采集信号，然后通过 Information Processing 部分进行信息的处理，最后在 Intelligence 部分对系统进行智能控制。该项目选取了其中的信息处理部分的一小部分，将应用进行解耦与模块化。将相对独立的功能封装进 Unikernel 运行，来发挥 Unikernel 快速，安全，轻量的优点，满足相应需求。相关成果如下：

1. 选用支持多种语言的 OSv 作为 Unikernel 内核，并在此基础上对 OSv 中相关参数进行了修改，从而提升了 CPU 性能。
2. 使用 Go 语言实现一个轻量的 OSv 管理工具 Uigniter，功能包括创建、启动、停止 OSv 实例。详细内容见[Uigniter 文档](https://github.com/richardlee159/uigniter/tree/e1c063341d658ec897a029b30874bc01bb852a1a)。

​		Unikernel 的解决方案具有容器方案所没有的隔离性、安全性、多进程/线程方案所没有的低延迟、轻量性、高容错率与模块解耦的特性，在未来 IoT 互联领域有着相当不错的前景。





### 2018年

#### X-Doudou 小组

##### 项目简介

​		该项目设计并初步实现了一个面向开发人员和系统管理人员的平台 Cunik ，用于方便地构建、分发、运行、管理 Unikernel 应用。Cunik 的设计目标是克服 Unikernel 配置难、部署繁琐的缺点，同时发挥 Unikernel 隔离性好、性能优良的特点，使运维人员轻松地获益于 Unikernel 这一新兴的技术。

关键词： Unikernel 、虚拟化、容器化

参考项目：libvirt、Rumprun 、OSv 



##### 项目可行性分析

- Unikernel 在保持了原有的安全性、隔离性、易部署性的前提下，还做到了在启动速度、运行速度、内存开销等方面全面胜过 Docker。Unikernel 可以在不同的硬件平台上用不同的方法实现不同的应用程序，现在 Unikernel 正运行在世界各地的研究实验室、服务器机房以及各种低功耗设备上。
- Cunik 向用户隐藏繁琐的细节，使用户可以轻松地构建、分发、获取和配置 Unikernel 应用，降低开发、部署和运维成本，并可以克服 Unikernel 开发难度高、分发部署困难、对系统管理人员要求高、对现有云计算架构改动大的缺点。借助 Unikernel 的优势，Cunik 可以使用户轻松获得显著的性能提升和更高的安全性、减小攻击面、降低资源占用。

- libvirt 提供了便捷且功能强大的虚拟机管理工具。可以基于 libvirt 构建 Cunik-engine 的 VM Backends 和 VM Hypervisor 部分从而方便管理虚拟机。




##### 项目困难点分析

- 需要基于现有的 Unikernel 应用重新开发所需要的平台。



##### 项目成果分析

​		该小组通过 Python 完成了 Cunik-engine 和 Cunik-cli 的设计，并手动制作了包含 nginx(Rumprun)、redis(Rumprun) 和 redis(OSv) 的本地镜像仓库，用 Cunik 成功运行了这三种应用。最终在 redis(OSV) 这个应用上取得了比 Linux 上的原生进程更高的性能。

​		Cunik-engine 架构如下：

​		<img src=".\pictures\resp1.png" alt="cunik-engine" style="zoom: 50%;" />

​		其具体细节可参考[X-Doudou文档](https://github.com/OSH-2018/X-Doudou/tree/master/concluding-report)。调用 Cunik 后，程序会执行如下的内容：

```
1. 用户通过调用 Cunik API 中的 Creat、Run、Stop、Remove、Inspect 等 API 接口命令来启动 Cunik-engine。
2. Cunik-engine 在接受到命令后，首先会生成一个 Cunik Config，用于生成 Cunik Object。
3. 通过Cunik Models，engine 会生成 Cunik Object，并加入到 Cunik Registry 中，或对已有 Cunik Object 进行运行状态的修改。
4. 然后，Unikernel Backends 会根据不同的 Cunik Object 选择不同的 Unikernel 实现方式。
5. 接下来，根据所选择的 Unikernel 实现方式，并在 Image Regsitry 中查询 Unikernel 应用的 image ，然后由 VM Backends 生成 VM Config。
6. VM Hypervisor 接收 VM Config 并选择合适的虚拟机来运行这个 Unikernel 应用。
```

​		该项目目前只实现了对 kvm/qemu 虚拟机、Rumprun 和 OSv 两种 Unikernel 实现的简单支持。可以改进的内容包括：

- 整理当前 Cunik-engine 的架构；
- 实现对更多虚拟机平台以及 Unikernel 实现的支持；
- 持续支持新的 Unikernel 实现，并加入更多方便镜像打包与应用部署的特性，使其能够满足生产环境的需要；
- 更好的交互体验：实现在用户发出 Request 后，自动为用户选择最合适的一系列 Cunik 应用，达成从前端到后端的一键式搭建服务。



#### X-zos 小组

##### 项目简介

​		该项目设计了一个利用系统自带的虚拟网卡，通过socket和多线程并发收发调试信息的日志式调试系统 Umonitor，为运维人员对Unikernel的调试和维护工作提出了更为轻松有效的解决方案。用户在使用时只需在每个需要调试的 Unikernel 里调用工具中的 send_log() 函数，将想要得到的调试信息传入函数，然后在主机的环境里面启动一个host 端，host 端就能通过虚拟网卡接口接收到来自不同 Unikernel 的调试信息并整理保存。

关键词： Unikernel 、调试

参考项目： Rumprun 



##### 项目可行性分析

- 传统的调试手段在 Unikernel 上难以进行。包括：

  **通过与其他进程通信来进行追踪和调试**　

  ​		Unikernel 为了实现精简，而放弃了原有的很多功能，其中就包括多进程切换。没有了多进程，就无法利用与其他进程通信来进行debug。

  **编程过程中将信息输出在控制台或者文件中**

  ​		在实际运行 Unikernel 的时候是不会模拟显示器的，所以无法将调试信息输出到控制台。又因为 Unikernal 的文件系统做了很大的精简，没有VFS，而且不同 Unikernal 的文件系统设计也不完全一样,所以，我们如果将日志写入文件，就很难再将虚拟磁盘中的东西读出来。　　　　

- Unikernel 采用了比较原始的单地址空间方式，这可以简化了调试的难度。单地址空间有助于定位需要的信息在的位置，而并不会影响 Unikernel 的性能。



##### 项目困难点分析

- 可能实现的方案选择很多，包括文件I/O，串口通信，网络通信等。如何选择最合适的方案需要一定的时间与试错成本。本实验最终选择了通过网络完成unikernel向host发送日志信息的过程。



##### 项目成果分析

​		该小组设计的 Umonitor 已经可以在 rumpkernel 的平台上通过对 Unikernel 源代码的修改，通过网络通信的方式将 Unikernel 中我们想要的调试信息输出到制定文件中，初期制定目标已经达到。项目的优势包括：

1. **并发性**：只需启动一个host端就能服务复数的 Unikernel 而无需多开，提高了效率；
2. **兼容性**：避开了不同的 Unikernel 的差异性，如使用的语言，内存空间，文件系统等的不同，选择了它们的共性，对 socket 的支持作为实现方法，几乎所有的 Unikernel 都能无难度地移植这个调试系统；
3. **高度可控可定制化**：直接在运行 Unikernel 的虚拟机的模拟 vga 输出界面打印调试信息会造成很大的切换和检索的麻烦，而重定向 vga 输出信息至某个文件会输出非常多 Unikernel 自带对调试无用或者不够清晰的信息，不能得到一个组织良好的日志文件。此外，多个 Unikernel 并发重定向在某些情况下可能造成输出混杂，不能正确地输出文件。利用 socket 传递自己想要的调试信息并组织保存，能够生成用户自己最需要的最有用的日志文件，提高调试的效率。

项目可以改进的方向包括：

1. Unikernel 的调试工具必然需要提供一个通用的接口以实现对不同种类 Unikernel 的支持。目前的 Umonitor 已经实现了能同时对多个 Unikernel 的调试，所以下一步的目标可以是实现对多种 Unikernel 的通用接口。使其够方便的支持现阶段较为成熟的 Unikernel 实现的同时也能够通过用户友好的配置界面对其他 Unikernel 进行支持。
2. Umonitor 在运行之后实际上仍然只能被动地接受被调试的 Unikernel 输出的调试信息，这样虽然能够在一次设置后找到对应的错误信息出现的位置，但想要在 Unikernel 运行中途添加调试信息输出或者更进一步的设置断点和逐句执行都还做不到。可以考虑添加交互式调试功能。