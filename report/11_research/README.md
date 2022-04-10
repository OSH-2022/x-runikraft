***本文件仅供参考，最终报告是`research-report.tex`，[Release](https://github.com/OSH-2022/x-runikraft/releases/tag/v0.0.1.2-predev)中有相应的PDF文件。***

# Runikraft 调研报告

## 项目简介

Runikraft 是用Rust语言编写的能在RISC-V架构 + QEMU平台上运行unikernel。它基于用C语言实现的Unikraft，在继承Unikraft的高效性和可定制性的同时，进一步简化了构建系统镜像的流程，加入了RISC-V支持，并且用Rust语言提供了更强的内核安全保证。

## 项目背景

### 操作系统的架构

简单结构。

分层设计。

微内核。

可加载内核模块。

### 虚拟化

在1960s，大型机的运算速度已经远超过了人类的操作速度。为了充分利用大型机的算力，一台大型机配置了多个终端，多个用户可以同时通过终端与大型机交互，多个用户的程序轮转使用CPU时间。由于轮转速度很快，用户无法感受到自己的程序没有连续运行，所以在每个用户看来，他都拥有一台计算机。这种运行在大型机上的分时系统是最初的虚拟化。然而，随着个人计算机的发展，这种基于分时系统的虚拟化逐渐没落。虚拟化的再次兴起得益于互联网技术的发展和云计算的兴起，在云计算中，用户通过网络操作远程的虚拟机，这些远程虚拟机可以向外界提供网络服务。

#### 虚拟机

广义上的虚拟机是模拟硬件或解释高级语言的程序，比如Apple在两次macOS架构迁移时分别推出的Rosetta和Rosetta 2就是硬件模拟器，CPython就是高级语言解释器，而OpenJDK JRE可以视为硬件模拟器，只不过它模拟的是不存在的硬件。这三个示例都侧重协助运行程序，而几乎没有隔离措施。狭义的虚拟机是一个模拟完整的计算机系统的程序，在虚拟机上运行的系统看来，虚拟机和物理机没有明显区别，而且，虚拟机上运行的系统不能随意访问宿主机的资源，Virtual Box、WMWare就是这类虚拟机。由于云计算平台上运行的用户程序对计算资源的提供商并不可信，所以，提供商希望将用户程序与系统的其他部分隔离。这需要狭义的虚拟机。

最初的云计算服务就是向用户提供一台完整的远程虚拟机。通常的远程虚拟机帮助用户提供网络服务，也就是作为服务器。服务器其实不需要每时每刻都保持运行，而只需要在有人请求这项网络服务时运行，但是为了确保网络服务的可用性，服务器必须保持开机，用户必须持续为这台作为服务器的远程虚拟机付费。为了更细粒度地分配计算资源，云计算服务的提供商推出了serverless服务。在serverless中，原本的服务器被拆分成若干“函数”，其实也就是一个响应网络请求的程序。当网络服务被请求时，这个程序被启动，响应这个请求，然后退出。这需要一台虚拟机能够快速启动，可是传统的虚拟机无法满足要求。

#### 容器

一种解决方案是不使用虚拟机，而使用更加轻量的方式实现隔离，比如容器。以Docker为代表的传统容器是为了便捷地打包程序及其依赖诞生的，而并不强调隔离性。传统容器使用 Namespace/Cgroup 实现，这套容器技术实际上是从进程调度的角度入手，对内核进行的功能扩展。优势上来说，操作界面很 Linux、很方便，开销也很低，可以被用来无负担地套在已有应用外面来构建隔离的环境。并且它是纯软件方案，不和其他层面的物理机、虚拟机相冲突。然而，随着容器技术的不断发展，传统容器隔离性不足的缺陷逐渐暴露了出来。Namespace/Cgroup 是内核的一个部分，其中运行的容器仍然使用主机的 Linux 内核，它解决不了Linux内核中隔离性差的问题，攻击者可以利用Linux内核的漏洞实现容器逃逸，然后便可以直接对宿主机进行攻击。

基于操作系统本身的容器机制没办法解决安全性问题，需要一个隔离层。
而虚拟机是一个现成的隔离层，AWS这样的云服务已经让全世界相信，
对用户来说，“secure of VM” 是可以满足需求的。
虚拟机里面只要有个内核，就可以支持 OCI 规范的语义，
在内核上跑个 Linux 应用并不太难实现。所以，安全容器的隔离层让应用的
问题——不论是恶意攻击，还是意外错误——都不至于影响宿主机，
也不会在不同的 Pod 之间相互影响。而且实际上，额外隔离层带来的影响并不仅是安全，
对于调度、服务质量和应用信息的保护都有好处。目前的安全容器有两个主流实现：

- [Kata Container](https://github.com/kata-containers/kata-containers) 是MicroVM的一个经典的实现，它提供了一个MicroVM，并且有专门提供给 Kubernetes 的接口，有比较好的安全性和运行效率，现在已经开始逐步使用。但是其启动时间和内存占用与传统容器还有一定的差距。
- [gVisor](https://github.com/google/gvisor) 是基于进程虚拟化的容器实现，它拥有很好的隔离性，很小的内存占用和启动时间，但是系统调用效率不高。

容器虽然解决了传统虚拟机启动时间长的问题，但是无法兼顾效率和隔离性。

#### Unikernel

Unikernel在MicroVM的基础上更进一步，它放弃了运行在虚拟机上的系统内的隔离，让用户程序和系统程序运行在同一个地址空间下，用户通过函数调用（如`call`指令）而不是软中断或陷入（如`int`、`syscall`、`ecall`等指令）使用系统提供的服务，这免去了上下文切换的开销，大幅提高了系统调用的效率。高效的系统调用甚至使unikernel的响应时间和吞吐率优于容器。由于unikernel本质上是运行在虚拟上的独立操作系统，它拥有良好的隔离性。Unikernel的系统镜像中只包含了用户程序需要的代码，这使unikernel的镜像非常轻量，甚至比Docker镜像还小。

然而，为了追求轻量性，unikernels裁剪了传统的操作系统的众多组件，因此unikernels无法提供许多常用的库的应用程序接口，所以为了将现有的程序移植到某个unikernel平台，开发者不得不根据该unikernel的API重构程序。此外，为了轻量、快速，unikernels没有启用许多基本的并且不会影响性能的安全措施，这导致unikernels相比容器更容易受到用户程序的安全漏洞的影响。

目前的Unikernel 按实现方式大致可以分为两种：

- 全新的方式(Clean-slate)：在构建单一用途的操作系统的假设下，自由地使用现代工具来进行构建，比如模块化(modularity)、声明性代码(declarative code)、避开样板文件(avoiding boilerplate)等。并且从头开始思考操作系统和应用程序层的实现，使用高级语言进行系统库的编写，从而使得实现更加可掌控，得到的系统库质量更高。用 OCmal 语言编写的 MirageOS 就是使用 Clean-slate 方式实现的。
- 传统的方式(Legacy)：在不进行修改或只进行一些小的修改的前提下，运行现有的软件。这通常通过将现有的操作系统代码库重构到库操作系统中来实现。用 C 语言编写的 Rumprun 是使用 Legacy 方式实现的。

### Unikernels 面临的问题
Unikernel是为了解决容器的隔离性差和传统的虚拟机启动慢而诞生的，所以unikernels
必须做到启动快、延迟低、吞吐量大。Unikernels的目标是取代容器，成为云计算领域的
最佳选择，所以它们必须提供高效的网络支持。为了方便现有的程序移植到unikernels上，
unikernel应该考虑兼容性问题，它们应该以最小的代价提供目前常用的系统APIs，并且
移植目前常用的库。此外，unikernels镜像的构建不应该过于繁琐。

### 知名的 Unikernels

ClickOS。

MirageOS。

IncludeOS。

RustyHermit。

Rumprun。

Nanos。

Unikraft。

## 立项依据

我们调研的unikernel项目的不足之处可以概括为（并不每个unikernel都有所有缺点）：

- 系统内的组件耦合度过高，系统不易裁剪或拓展。在拓展方面做得比较好的unikernels有MirageOS、Rumprun和Unikraft。

- 需要使用专用的工具构建系统镜像。对于只有几个源文件的小型项目，使用专用的工具并不是什么大问题，但是，对于由成千上万个源文件组成的大型项目，更改构建环境本身就是一项浩大的工程。目前，Nanos允许用户程序使用独立的工具构建，MirageOS和RustHermit的构建工具是它们的语言的默认工具。

- 将安全性与隔离性等同，忽视了单个unikernel虚拟机的安全。目前，在文档中明确提到安全措施的unikernels有RustyHermit、Nanos和Unikraft。

- 核心代码使用不安全的程序设计语言编写。使用安全的编程语言写的unikernels只有MirageOS和RustyHermit。用不安全的程序设计语言难以避免实现时引入的安全漏洞。

- 不支持RISC-V架构。目前只有Nanos支持RISC-V架构。

虽然说兼容性也是unikernels的一大不足，但是许多unikernels的开发者已经在尽力解决它，以至于除了MirageOS外的unikernels都提供了或多或少的C标准库支持。

Nanos在兼容性、易构建性方面做得都很好，也支持RISC-V架构，可是它似乎为了支持直接运行ELF文件，抛弃了unikernels的最重要的无系统调用的特性；Unikraft声称可配置为POSIX兼容，而且系统架构比较清晰，可惜它不支持RISC-V，而且镜像的构建需要专用工具。Nanos和Unikraft的共同缺陷是使用不安全的C语言编写。MirageOS使用安全的语言编写，并且有丰富的软件包资源，可是完全不支持现有的程序；想将程序移植到MirageOS上必须用OCaml重构程序。RustHermit是对用C语言实现的HermitCore的重构，我们受它的启发，也决定用Rust重构一个现有的unikernel。

我们小组计划仿照Unikraft的架构，用Rust语言编写能在RISC-V架构+ QEMU平台上运行的unikernel——Runikraft。Runikraft的核心代码使用Rust编写，但允许用户代码使用任何语言编写
Runikraft强调构建系统镜像的简洁，用户只需要修改现有的项目的编译参数就可以构建基于Runikraft的系统镜像，而不必使用专用的工具链，更不需要重构代码。具体的方法是先将Runikraft编译成静态库，然后通过修改编译参数，让用户程序不链接标准库，而链接到Runikraft。

Runikraft支持内存管理、进程调度、进程通信和磁盘管理，而且这些功能都是是可选的且可拓展的，如果用户不需要某项功能，他可以不将相关模块打包进系统镜像中，如果用户能够提供某些功能的更好实现，他可以用自己的实现替换原有的模块。

Runikraft的亮点：

- 用安全的Rust语言编写；
- 支持正在迅速发展的RISC-V指令集架构；
- 构建流程简单；
- 模块化设计，在保持unikernel的高效的同时降低维护难度。

Runikraft的目标是提供比较完整的POSIX兼容层、Linux兼容层和C标准库，但是考虑到时间有限，这些功能只能部分实现。

我们考虑过改善unikernel的调试，即zos小组的研究，但是我们发现，QEMU事实上已经支持交互式的虚拟机调试了。

我们考虑过但最终不打算实现与Linux的二进制兼容，即unipanic小组的研究，因为我们认为不会出现需要移植无法获得源代码的程序的情况：

- 如果源代码因著作权问题无法获取，
    - 修改二进制文件移植，即把`syscall`指令替换为`jmp` 指令后移植。这样可以保持效率，但会侵犯著作权。
    - 直接移植未修改的二进制文件。这需要\texttt{syscall}，会引入上下文切换开销，效率难以保证。

- 如果源代码因软件无人维护无法获取，那这样的过时软件本身就不应该被继续使用。

在系统架构方面，我们将主要参考Unikraft；在技术方面，我们将参考RustyHermit、Nanos和Chen and Wu的 *rCore Tutorial Book*。

## 前瞻性/重要性分析

### 使用先进的工具构建

Rust和RISC-V都是新兴事物，它们都是在吸取旧事物的教训的基础上诞生的，而且，实践表明，两者都正在经历蓬勃的发展，并正在分别逐步取代旧事物。因此，用Rust在RISC-V上开发unikernel顺应了历史的趋势。

Rust 是由 Mozilla 研究室主导开发的一门现代系统编程语言，自 2015 年 5 月发布 1.0 之后，一直以每 6 周一个小版本的开发进度稳定向前推进。语言设计上跟 C++ 一样强调零开销抽象和 RAII。拥有极小的运行时和高效的 C 绑定，使其运行效率与 C/C++ 一个级别，非常适合对性能要求较高的系统编程领域。利用强大的类型系统和独特的生命周期管理实现了编译期内存管理，保证内存安全和线程安全的同时使编译后的程序运行速度极快，Rust 还提供函数式编程语言的模式匹配和类型推导，让程序写起来更简洁优雅。总地来说，Rust是一门赋予每个人 构建可靠且高效软件能力的语言。Rust具有高性能、可靠性、生产力三方面的优势。

RISC-V是于2010年诞生自加州大学伯克利分校的精简指令集架构，它的目标是成为一个通用的指令集架构，它能适应包括从最袖珍的嵌入式控制器，到最快的高性能计算机等各种规模的处理器；它能兼容各种流行的软件栈和编程语言；它能适应所有实现技术，包括现场可编程门阵列(FPGA) 、专用集成电路(ASIC) 、全定制芯片，甚至未来的设备技术；它对所有微体系结构样式都有效，例如微编码或硬连线控制、顺序或乱序执行流水线、单发射或超标量等；它支持广泛的专业化，成为定制加速器的基础；它是稳定的，基础的指令集架构不应该改变。与以往的ISA不同，RISC-V是*模块化*的。它的核心是一个名为RV32I的基础ISA，运行一个完整的软件栈。RV32I是固定的，永远不会改变。这为编译器编写者，操作系统开发人员和汇编语言程序员提供了稳定的目标。模块化来源于可选的标准扩展，根据应用程序的需要，硬件可以包含或不包含这些扩展。这种模块化特性使得RISC-V具有了袖珍化、低能耗的特点，而这对于嵌入式应用可能至关重要。RISC-V在设计时考虑了成本、简洁性、性能、架构和具体实现的分离、提升空间、程序大小和易于编程/编译/链接七个方面的因素。

目前的unikernel中，使用/支持两者中的一个的都很少，而根本没有将两者结合者。Runikraft的亮点之一就是将两者结合。

### 模块化设计

目前的大多数unikernel强调“uni-”，它们的设计者认为这样有利于提高效率，所以系统被设计成了一个整体，这个整体向用户提供能够调用函数。具体的表现就是系统的源代码堆在一起，ClickOS、IncludeOS、MirageOS、RustyHermit都有这样的问题。系统缺乏明确的功能组件，所以系统必须作为一个整体维护。

在Runikraft中，只有极少数平台层的代码被放到了系统的核心组件中，而调度器、分配器等组件一律是micro-libraies。这些micro-libraires遵循一套明确定义的APIs，同一个系统模块可以有多种实现，用户可以轻松为自己的需求选择合适的系统组件的实现。从Unikraft给出的基准测试数据看，这种模块划分不会降低系统的效率。

## 相关工作

### 安全容器

《项目背景》中已指出，容器是云计算领域的常用的隔离手段，而由于容器并不是沙盒，
它提供的隔离能力不足以运行潜在的恶意代码，安全容器应运而生。从设计目标看，安全
容器的隔离能力与unikernels等同。安全容器的目标是能够直接运行ELF二进制文件，
而unikernels通常要求从源代码重新编译。

Kata Containers和gVisor都使用Go语言实现。

Kata Containers的实现思路是轻量级虚拟机，它是容器向uniKernel的过渡。它的主要特点是：


- **安全** Runs in a dedicated kernel, providing isolation of network, I/O and memory and can utilize hardware-enforced isolation with virtualization VT extensions.
- **兼容** Supports industry standards including OCI container format, Kubernetes CRI interface, as well as legacy virtualization technologies.
- **高效** Delivers consistent performance as standard Linux containers; increased isolation without the performance tax of standard virtual machines.
- **简洁** Eliminates the requirement for nesting containers inside full blown virtual machines; standard interfaces make it easy to plug in and get started.

gVisor的实现思路是半虚拟化操作系统，它在用户空间运行，以拦截系统调用的方式为应用程序提供服务。它与传统容器的关键区别是没有简单地将应用程序的系统调用重定向给宿主机内核，而是实现了大多数内核原语，并基于这些原语实现系统调用。gVisor与unikernel的区别是gVisor没有模拟硬件，而只是模拟了一个Linux内核；unikernel本身是一个运行在虚拟硬件上的操作系统。gVisor的特点：

- **容器原生安全** By providing each container with its own application kernel, gVisor limits the attack surface of the host. This protection does not limit functionality: gVisor runs unmodified binaries and integrates with container orchestration systems, such as Docker and Kubernetes, and supports features such as volumes and sidecars.

- **资源高效** Containers are efficient because workloads of different shapes and sizes can be packed together by sharing host resources. gVisor uses host-native abstractions, such as threads and memory mappings, to co-operate with the host and enable the same resource model as native containers.

- **跨平台** Modern infrastructure spans multiple cloud services and data centers, often with a mix of managed services and virtualized or traditional servers. The pluggable platform architecture of gVisor allows it to run anywhere, enabling consistent security policies across multiple environments without having to rearchitect your infrastructure.

### 嵌入式系统

大部分unikernels只打算在虚拟机上运行，但是IncludeOS和Rumprun支持在嵌入式设备上运行，
往年的ridiculous-includeos小组也做过将unikernel移植到嵌入式设备的研究。可见嵌入式设备也是unikernel潜在的应用领域。嵌入式系统的类型丰富多样，这里着重介绍物联网（IoT）系统。

目前，物联网操作系统主要分为两大类，一是由传统的嵌入式实时操作系统(RTOS)发展而来，比如FreeRTOS、LiteOS、RT-Thread；二是由互联网公司的云平台延伸而来，基于传统操作系统进行“剪裁”和定制的IoT OS，比如Ali OS Things、TencentOS tiny、Win10 IOT。

[TencentOS Tiny](https://github.com/OpenAtomFoundation/TencentOS-tiny) 是腾讯面向物联网领域开发的实时操作系统，具有低功耗，低资源占用，模块化，安全可靠等特点，可有效提升物联网终端产品开发效率。

TencentOS tiny 提供精简的 RTOS 内核，内核组件可裁剪可配置，可快速移植到多种主流 MCU (如 STM32 全系列)及模组芯片上。而且，基于 RTOS 内核提供了丰富的物联网组件，内部集成主流物联网协议栈（如CoAP/MQTT/TLS/DTLS/LoRaWAN/NB-IoT等），可助力物联网终端设备及业务快速接入腾讯云物联网平台。

[MS-RTOS](https://github.com/ms-rtos) (Micro Safe RTOS) 是翼辉信息全新设计的一款面向未来的安全实时操作系统，其最大的特点是开创性地在没有 MMU 和资源受限的 MCU（如Cortex-M3）上也能支持多进程与动态装载技术，使得应用与系统能分离开发、独立升级；MS-RTOS 支持内核空间内存保护（应用程序通过 syscall 访问内核），使得内核有着非常高的安全性。MS-RTOS 在提供足够丰富功能的同时，保持了高效简洁的实现，对 ROM、RAM 消耗极低，特别适用于对硬件成本敏感、安全性要求特别高的产品。

### 用 Rust 编写的操作系统

Rust语言的目标之一就是取代C语言，成为用于系统开发的底层语言。目前已有大量用Rust开发的操作系统：

| 名称            | 架构            | 纯 Rust | 活跃 | 内核架构                 | 目标          | 用户态 | GUI  | 贡献者数 | 文件系统      | 许可          |
| --------------- | --------------- | ------- | ---- | ------------------------ | ------------- | ------ | ---- | -------- | ------------- | ------------- |
| **redox**       | x86 and x86_64  | 是      | 是   | 微内核                   | 通用          | 是     | 是   | 50       | ZFS, RedoxFS  | Expat         |
| **Theseus OS**  | x86_64, ARM WIP | 是      | 是   | Safe-language SAS/SPL OS | 通用 + 嵌入式 |        | 是   | 25       | Custom, FAT32 | Expat         |
| **Tock**        | Cortex M        |         | 是   |                          |               |        | 否   | 40       |               | APL 2 / Expat |
| **intermezzOS** | x86_64          | 否      | 是   | ?                        | PoC           | 否     | 否   | 18       | no            | APL 2 / Expat |
| **RustOS**      | i386            | ?       | 是   | 无                       | PoC           | 否     | 否   | 10       | no            | APL 2 / Expat |
| **rustboot**    | i386            | ?       | 否   | 无                       | PoC           | 否     | 否   | 8        | no            | Expat         |

其中的Redox是一款功能完整的类UNIX操作系统，而不只是操作系统内核，它包含C标准库、
窗口管理器、浏览器、文本编辑器、图像查看器、终端模拟器等众多软件包。
