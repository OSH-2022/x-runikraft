# 结题报告

## 工作摘要

我们小组的项目是用 Rust 改写 Unikraft。我们实现的 crates 有 `rkalloc`（分配器的 API）、`rkallocbuddy`（伙伴分配器）、`rkargparse`（命令行参数分析）、`rkboot`（引导器）、`rkgpu`（显示设备驱动）、`rkinput`（输入设备驱动）、`rklock`（互斥体和信号量）、`rkmpi`（基于信箱的线程通信）、`rkring`（无锁的环形缓冲区）、`rkshed`（调度器的 API）、`rkshedcoop`（支持 SMP 的协作式的 FCFS 调度器）、`rkschedpreem`（支持 SMP 的抢占式的 RR 调度器）、`rkswrand`（密码学安全的随机数生成器）、`rktimeconv`（时间格式转换）。我们支持的设备有 ns16550、goldfish-rtc、virtio-blk、virtio-console、virtio-rng、virtio-gpu、virtio-input、virtio-net。我们还实现了基于 kconfig 的功能定制系统。
