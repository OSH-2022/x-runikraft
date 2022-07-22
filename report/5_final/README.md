***本文件仅供参考，最终报告是`final-report.tex`，[Release](https://github.com/OSH-2022/x-runikraft/releases/tag/v0.1.0.0-alpha)中有相应的PDF文件。***

# 结题报告

## 工作摘要

我们小组的目标是用 Rust 改写 Unikraft，并且支持 RISC-V 架构。 与 Unikraft 一样，Runikraft 强调系统的可定制性，在 Runikraft 中，几乎所有的功能都由独立的 crates 实现，这使我们能够降低内核的各个模块之间的耦合度，使用户能够轻松地定制 OSes。这种设计也使 Runikraft 的组件可以被独立地更新。经过两个月的不懈努力，我们最终实现了支持对称多处理器的抢占式的 RR 调度器、 密码学安全的随机数生成器、显示设备驱动、信号量原语、基于信箱的IPC等crates，我们支持的设备有 ns16550、goldfish-rtc、virtio-blk、virtio-console、virtio-rng、virtio-gpu、virtio-input、virtio-net。我们还实现了基于 kconfig 的功能定制系统。为了展示 Runikraft，我们写了基于 Runikraft 的数独程序，它拥有简单的图形界面，使用了 `rkallocbuddy`、`rkschedpreem`、`rklock`、`rkgpu` 等 crates，能够比较全面地展示 Runikraft 的功能。在使用 release 配置构建时，数独程序的镜像大小只有 100KiB。

