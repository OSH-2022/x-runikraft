<!--本文件仅供大家参考，不写到最终报告中-->

# Unikraft 源代码分析 

核心代码位于`include`、`arch`、`plat`。`include`是API。`plat`包含与平台相关的代码，其中的`plat\drivers`有我们可能需要使用的虚拟硬件的驱动程序。`arch`包含与CPU架构有关的代码，比如设置内存屏障，原子操作。

`lib`里是micro-libraries。每个库的公开API位于`include`目录，实现位于库的根目录。

## Micro-libraries

### `uknetdev`

uknetdev提供了一套网络驱动与网络栈的实现或底层的网络程序之间的通用接口。大部分的接口接受一个网络设备指针（`struct uk_netdev*`）参数，而它可以通过调用`un_netdev_get()`获取。

以下是初始化该API的顺序：

1. `uk_netdev`
2. `uk_netdev_configure()`
3. `uk_netdev_txq_configure()`
4. `uk_netdev_rxq_configure()`
5. `uk_netdev_start()`

初始化完成后，才能调用传输和接受函数。

网络设备的4个状态：

1. `UK_NETDEV_UNREGISTERED`
2. `UK_NETDEV_UNCONFIGURED`
3. `UK_NETDEV_CONFIGURED`
4. `UK_NETDEV_RUNNING`

#### 依赖：

| 配置                            | 库         | 文件                                                         |
| ------------------------------- | ---------- | ------------------------------------------------------------ |
|                                 | 核心       | refcount.h、essentials.h、list.h                             |
|                                 | ukdebug    | assert.h、print.h                                            |
|                                 | ukalloc    | alloc.h                                                      |
|                                 | uklibparam | libparam.h                                                   |
| `LIBUKNETDEV_DISPATCHERTHREADS` | uksched    | sched.h                                                      |
| `LIBUKNETDEV_DISPATCHERTHREADS` | uklock     | semaphore.h                                                  |
|                                 | nolibc     | sys/types.h、inttypes.h、stddef.h、limits.h、errno.h、stdio.h、stdlib.h、string.h |

#### 头文件：

1. `netbuf.h`: 网络缓冲区，`uk_netbuf`结构体和上面的一系列操作。
2. `netdev_core.h`: uknetdev API的公共声明，定义了API中所有的类型。
3. `netdev_driver.h`
4. `netdev.h`: 包含uknetdev的所有接口。

#### 导出的符号：

```
uk_netbuf_init_indir
uk_netbuf_alloc_indir
uk_netbuf_alloc_buf
uk_netbuf_prepare_buf
uk_netbuf_free_single
uk_netbuf_free
uk_netbuf_disconnect
uk_netbuf_connect
uk_netbuf_append
uk_netdev_drv_register
uk_netdev_count
uk_netdev_get
uk_netdev_id_get
uk_netdev_drv_name_get
uk_netdev_state_get
uk_netdev_info_get
uk_netdev_einfo_get
uk_netdev_rxq_info_get
uk_netdev_txq_info_get
uk_netdev_configure
uk_netdev_rxq_configure
uk_netdev_txq_configure
uk_netdev_start
uk_netdev_hwaddr_set
uk_netdev_hwaddr_get
uk_netdev_promiscuous_get
uk_netdev_promiscuous_set
uk_netdev_mtu_get
uk_netdev_mtu_set
uk_netdev_rxq_intr_enable
uk_netdev_rxq_intr_disable
```

### `ukblkdev`

与uknetdev类似，ukblkdev提供了一套块设备驱动和需要与块设备直接通信的底层设备的通用接口。

初始化顺序：

1. `uk_blkdev_configure()`
2. `uk_blkdev_queue_configure()`
3. `uk_blkdev_start()`

块设备的4个状态：

1. `UK_BLKDEV_UNREGISTERED`
2. `UK_BLKDEV_UNCONFIGURED`
3. `UK_BLKDEV_CONFIGURED`
4. `UK_BLKDEV_RUNNING`

#### 依赖：

| 配置                                   | 库      | 文件                                                         |
| -------------------------------------- | ------- | ------------------------------------------------------------ |
|                                        | 核心    | arch/types.h、arch/atomic.h、list.h、errptr.h、bitops.h、ctors.h |
|                                        | ukdebug | assert.h、print.h                                            |
|                                        | ukalloc | alloc.h                                                      |
| `CONFIG_LIBUKBLKDEV_DISPATCHERTHREADS` | uksched | sched.h                                                      |
| `CONFIG_LIBUKBLKDEV_DISPATCHERTHREADS` | uklock  | semaphore.h                                                  |
|                                        | nolibc  | fcntl.h、sys/types.h、stdint.h、stdio.h、errno.h、limits.h、string.h、stdlib.h、inttypes.h |

#### 头文件：

1. `blkreq.h`
2. `blkdev_core.h`
3. `blkdev_driver.h`
4. `blkdev.h`

#### 导出的符号：

```
uk_blkdev_drv_register
uk_blkdev_count
uk_blkdev_get
uk_blkdev_id_get
uk_blkdev_drv_name_get
uk_blkdev_state_get
uk_blkdev_get_info
uk_blkdev_configure
uk_blkdev_queue_get_info
uk_blkdev_queue_configure
uk_blkdev_start
uk_blkdev_queue_submit_one
uk_blkdev_queue_finish_reqs
uk_blkdev_sync_io
uk_blkdev_stop
uk_blkdev_queue_unconfigure
uk_blkdev_drv_unregister
uk_blkdev_unconfigure
```

### `ukboot`

完成初始化工作，调用用户定义的`main`函数。

核心库的`plat/bootstrap.h`中声明了函数`void ukplat_entry_argp(char *arg0, char *argb, __sz argb_len)`（接受未分析的参数）和`void ukplat_entry(int argc, char *argv[])`（接受已分析的参数），与平台和架构相关的代码最后会跳转到`ukplat_entry_argp`函数，由`ukboot`完成通用的初始化。

#### Unikraft的开机流程（KVM+AMD64）：

1. bootloader（由虚拟机的固件实现）。
2. `_libkvmplat_start32`（位于`plat/kvm/x86/entry64.S`）
    1. enable pae；
    2. enable long mode；
    3. load pml4 pointer；
    4. enable paging；
    5. `jmp _libkvmplat_start64`。
3. `_libkvmplat_start64`
    1. 一系列复杂的初始化；
    2. `call _libkvmplat_entry`。
4. `_libkvmplat_entry`（位于`plat/kvm/x86/setup.c`），它接受`struct multiboot_info *`参数
    1. `_init_cpufeatures`；
    2. `_libkvmplat_init_console`；
    3. `traps_init`；
    4. `intctrl_init`；
    5. `_mb_get_cmdline`：把multiboot_info中的命令行参数复制到数据段的`char cmdline[8192]`；
    6. `_mb_init_mem`：处理与内存分段有关的操作；
    7. `_mb_init_initrd`；
    8. (`CONFIG_HAVE_SMP`)`acpi_init`；
    9. (`CONFIG_HAVE_SYSCALL`)`_init_syscall；`
    10. (`CONFIG_HAVE_X86PKU`)`_check_ospke`；
    11. 调用`_libkvmplat_newstack`从引导栈切换走。
        可以看出，AMD64的开机流程很复杂，在ARMv8中，系统的起点是用汇编语言编写的`_libkvmplat_entry`，它调用用C语言编写的`_libkvmplat_start`，之后的流程与AMD64相同。
5. `_libkvmplat_entry2`：把cmdline传递给`ukplat_entry_argp`。
6. `ukplat_entry_argp`：把命令行参数拆成argc+argv的形式，然后调用`ukplat_entry`。
7. `ukplat_entry`
    1. 调用`ctorfn`注册的构造函数（`ctors.h`提供了注册构造函数的机制）；
    2. (`CONFIG_LIBUKLIBPARAM`)进一步分析命令行参数；
    3. (`!CONFIG_LIBUKBOOT_NOALLOC`)依次尝试在每一块内存区域上初始化分配器（`uk_<name>_init`），成功创建分配器后，将剩余的内存区域加入分配器（`uk_alloc_addmem`）；
    4. (`CONFIG_LIBUKALLOC`)初始化中断（`ukplat_irq_init`）；
    5. (`CONFIG_LIBUKSCHED`)初始化调度器`uk_sched_default_init`；
    6. (`CONFIG_LIBUKSCHED`?)
        - (true) 创建主线程（执行`main_thread_func`）→启动调度器（`uk_sched_start`），
        - (false)启动中断（`ukplat_lcpu_enable_irq`）→直接调用`main_thread_func`。
8. `main_thread_func`
    1. 调用init table上注册的函数（由`init.h`提供，与`ctorfn`类似）；
    2. (`CONFIG_LIBUKSP`)`uk_stack_chk_guard_setup`；
    3. 调用用户程序的构造函数`__preinit_array`和`__init_array`；
    4. 调用用户程序的`main`。
9. `main`。
10. 关机/崩溃。

#### 依赖：

| 配置                                           | 库            | 文件                                                         |
| ---------------------------------------------- | ------------- | ------------------------------------------------------------ |
|                                                | 核心          | essentials.h、plat/console.h、arch/lcpu.h、plat/bootstrap.h、plat/memory.h、plat/lcpu.h、plat/irq.h、plat/time.h、ctors.h、init.h |
|                                                | ukdebug       | print.h、                                                    |
| `CONFIG_LIBUKBOOT_INITBBUDDY`（5种分配器互斥） | ukallocbbuddy | allocbbuddy.h                                                |
| `CONFIG_LIBUKBOOT_INITREGION`                  | ukallocregion | allocregion.h                                                |
| `CONFIG_LIBUKBOOT_INITMIMALLOC`                | （未实现）    | mimalloc.h                                                   |
| `CONFIG_LIBUKBOOT_INITTLSF`                    | （未实现）    | tlsf.h                                                       |
| `CONFIG_LIBUKBOOT_INITTINYALLOC`               | （未实现）    | tinyalloc.h                                                  |
| `CONFIG_LIBUKSCHED`                            | uksched       | sched.h                                                      |
|                                                | ukargparse    | argparse.h                                                   |
| `CONFIG_LIBUKLIBPARAM`                         | uklibparam    | libparam.h                                                   |
| `CONFIG_LIBUKSP`                               | uksp          | sp.h                                                         |
|                                                | nolibc        | stdio.h、stddef.h、errno.h                                   |







