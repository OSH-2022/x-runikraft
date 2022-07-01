# Details

Date : 2022-06-29 17:10:55

Directory d:\\OSH\\x-runikraft

Total : 123 files,  15836 codes, 3324 comments, 3146 blanks, all 22306 lines

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)

## Files
| filename | language | code | comment | blank | total |
| :--- | :--- | ---: | ---: | ---: | ---: |
| [README.md](/README.md) | Markdown | 47 | 0 | 28 | 75 |
| [core/src/align_as.rs](/core/src/align_as.rs) | Rust | 28 | 4 | 4 | 36 |
| [core/src/bitcount.rs](/core/src/bitcount.rs) | Rust | 41 | 28 | 9 | 78 |
| [core/src/compat_list/list.rs](/core/src/compat_list/list.rs) | Rust | 153 | 31 | 24 | 208 |
| [core/src/compat_list/mod.rs](/core/src/compat_list/mod.rs) | Rust | 8 | 4 | 3 | 15 |
| [core/src/compat_list/slist.rs](/core/src/compat_list/slist.rs) | Rust | 83 | 22 | 18 | 123 |
| [core/src/compat_list/stailq.rs](/core/src/compat_list/stailq.rs) | Rust | 117 | 26 | 20 | 163 |
| [core/src/compat_list/tailq.rs](/core/src/compat_list/tailq.rs) | Rust | 202 | 38 | 28 | 268 |
| [core/src/config.rs](/core/src/config.rs) | Rust | 4 | 6 | 2 | 12 |
| [core/src/errno.rs](/core/src/errno.rs) | Rust | 100 | 106 | 2 | 208 |
| [core/src/lib.rs](/core/src/lib.rs) | Rust | 6 | 26 | 5 | 37 |
| [default_config.rs](/default_config.rs) | Rust | 23 | 10 | 7 | 40 |
| [dev-test/src/thread_context.rs](/dev-test/src/thread_context.rs) | Rust | 34 | 0 | 7 | 41 |
| [dev-test/src/trap.rs](/dev-test/src/trap.rs) | Rust | 9 | 0 | 3 | 12 |
| [dev-test/src/uart.rs](/dev-test/src/uart.rs) | Rust | 20 | 0 | 3 | 23 |
| [doc/assets/Runikraft_logo.svg](/doc/assets/Runikraft_logo.svg) | XML | 1,679 | 0 | 18 | 1,697 |
| [lib/rkalloc/alloc_error_handler.rs](/lib/rkalloc/alloc_error_handler.rs) | Rust | 7 | 4 | 4 | 15 |
| [lib/rkalloc/src/lib.rs](/lib/rkalloc/src/lib.rs) | Rust | 105 | 90 | 26 | 221 |
| [lib/rkalloc_buddy/src/debug.rs](/lib/rkalloc_buddy/src/debug.rs) | Rust | 28 | 4 | 4 | 36 |
| [lib/rkalloc_buddy/src/lib.rs](/lib/rkalloc_buddy/src/lib.rs) | Rust | 288 | 105 | 41 | 434 |
| [lib/rkalloc_buddy/support/find_data_size.cpp](/lib/rkalloc_buddy/support/find_data_size.cpp) | C++ | 38 | 5 | 5 | 48 |
| [lib/rkblkdev/src/blkdev.rs](/lib/rkblkdev/src/blkdev.rs) | Rust | 245 | 382 | 40 | 667 |
| [lib/rkblkdev/src/blkdev_core.rs](/lib/rkblkdev/src/blkdev_core.rs) | Rust | 88 | 109 | 24 | 221 |
| [lib/rkblkdev/src/blkdev_driver.rs](/lib/rkblkdev/src/blkdev_driver.rs) | Rust | 57 | 75 | 12 | 144 |
| [lib/rkblkdev/src/blkfront.rs](/lib/rkblkdev/src/blkfront.rs) | Rust | 22 | 73 | 6 | 101 |
| [lib/rkblkdev/src/blkreq.rs](/lib/rkblkdev/src/blkreq.rs) | Rust | 50 | 83 | 13 | 146 |
| [lib/rkblkdev/src/lib.rs](/lib/rkblkdev/src/lib.rs) | Rust | 83 | 45 | 17 | 145 |
| [lib/rkboot/src/lib.rs](/lib/rkboot/src/lib.rs) | Rust | 122 | 27 | 24 | 173 |
| [lib/rkgpu/src/lib.rs](/lib/rkgpu/src/lib.rs) | Rust | 133 | 33 | 15 | 181 |
| [lib/rkinput/src/lib.rs](/lib/rkinput/src/lib.rs) | Rust | 2 | 26 | 10 | 38 |
| [lib/rklock/src/lib.rs](/lib/rklock/src/lib.rs) | Rust | 7 | 26 | 7 | 40 |
| [lib/rklock/src/mutex.rs](/lib/rklock/src/mutex.rs) | Rust | 82 | 2 | 8 | 92 |
| [lib/rklock/src/semaphore.rs](/lib/rklock/src/semaphore.rs) | Rust | 47 | 1 | 6 | 54 |
| [lib/rkmbox/src/lib.rs](/lib/rkmbox/src/lib.rs) | Rust | 109 | 45 | 38 | 192 |
| [lib/rknetdev/src/lib.rs](/lib/rknetdev/src/lib.rs) | Rust | 5 | 26 | 5 | 36 |
| [lib/rknetdev/src/netbuf.rs](/lib/rknetdev/src/netbuf.rs) | Rust | 2 | 30 | 5 | 37 |
| [lib/rknetdev/src/netdev.rs](/lib/rknetdev/src/netdev.rs) | Rust | 0 | 0 | 2 | 2 |
| [lib/rknetdev/src/netdev_core.rs](/lib/rknetdev/src/netdev_core.rs) | Rust | 222 | 133 | 65 | 420 |
| [lib/rknetdev/src/netdev_driver.rs](/lib/rknetdev/src/netdev_driver.rs) | Rust | 15 | 45 | 9 | 69 |
| [lib/rkparam/src/lib.rs](/lib/rkparam/src/lib.rs) | Rust | 192 | 69 | 55 | 316 |
| [lib/rkplat/src/drivers/device_tree.rs](/lib/rkplat/src/drivers/device_tree.rs) | Rust | 261 | 61 | 67 | 389 |
| [lib/rkplat/src/drivers/mod.rs](/lib/rkplat/src/drivers/mod.rs) | Rust | 17 | 3 | 3 | 23 |
| [lib/rkplat/src/drivers/rtc/goldfish.rs](/lib/rkplat/src/drivers/rtc/goldfish.rs) | Rust | 52 | 4 | 11 | 67 |
| [lib/rkplat/src/drivers/rtc/mod.rs](/lib/rkplat/src/drivers/rtc/mod.rs) | Rust | 7 | 5 | 4 | 16 |
| [lib/rkplat/src/drivers/uart/mod.rs](/lib/rkplat/src/drivers/uart/mod.rs) | Rust | 7 | 6 | 4 | 17 |
| [lib/rkplat/src/drivers/uart/ns16550.rs](/lib/rkplat/src/drivers/uart/ns16550.rs) | Rust | 93 | 36 | 23 | 152 |
| [lib/rkplat/src/drivers/virtio/blk.rs](/lib/rkplat/src/drivers/virtio/blk.rs) | Rust | 214 | 92 | 28 | 334 |
| [lib/rkplat/src/drivers/virtio/console.rs](/lib/rkplat/src/drivers/virtio/console.rs) | Rust | 135 | 13 | 13 | 161 |
| [lib/rkplat/src/drivers/virtio/gpu.rs](/lib/rkplat/src/drivers/virtio/gpu.rs) | Rust | 410 | 47 | 60 | 517 |
| [lib/rkplat/src/drivers/virtio/hal.rs](/lib/rkplat/src/drivers/virtio/hal.rs) | Rust | 49 | 8 | 13 | 70 |
| [lib/rkplat/src/drivers/virtio/header.rs](/lib/rkplat/src/drivers/virtio/header.rs) | Rust | 149 | 108 | 58 | 315 |
| [lib/rkplat/src/drivers/virtio/input.rs](/lib/rkplat/src/drivers/virtio/input.rs) | Rust | 136 | 39 | 21 | 196 |
| [lib/rkplat/src/drivers/virtio/mod.rs](/lib/rkplat/src/drivers/virtio/mod.rs) | Rust | 62 | 38 | 21 | 121 |
| [lib/rkplat/src/drivers/virtio/net.rs](/lib/rkplat/src/drivers/virtio/net.rs) | Rust | 159 | 52 | 28 | 239 |
| [lib/rkplat/src/drivers/virtio/queue.rs](/lib/rkplat/src/drivers/virtio/queue.rs) | Rust | 190 | 46 | 32 | 268 |
| [lib/rkplat/src/lib.rs](/lib/rkplat/src/lib.rs) | Rust | 14 | 25 | 10 | 49 |
| [lib/rkplat/src/riscv64/bootstrap.rs](/lib/rkplat/src/riscv64/bootstrap.rs) | Rust | 91 | 29 | 21 | 141 |
| [lib/rkplat/src/riscv64/console.rs](/lib/rkplat/src/riscv64/console.rs) | Rust | 99 | 18 | 25 | 142 |
| [lib/rkplat/src/riscv64/constants.rs](/lib/rkplat/src/riscv64/constants.rs) | Rust | 1 | 8 | 3 | 12 |
| [lib/rkplat/src/riscv64/device.rs](/lib/rkplat/src/riscv64/device.rs) | Rust | 17 | 10 | 6 | 33 |
| [lib/rkplat/src/riscv64/exception.rs](/lib/rkplat/src/riscv64/exception.rs) | Rust | 47 | 6 | 3 | 56 |
| [lib/rkplat/src/riscv64/intctrl.rs](/lib/rkplat/src/riscv64/intctrl.rs) | Rust | 19 | 7 | 5 | 31 |
| [lib/rkplat/src/riscv64/irq.rs](/lib/rkplat/src/riscv64/irq.rs) | Rust | 57 | 26 | 12 | 95 |
| [lib/rkplat/src/riscv64/lcpu.rs](/lib/rkplat/src/riscv64/lcpu.rs) | Rust | 175 | 43 | 34 | 252 |
| [lib/rkplat/src/riscv64/mcause.rs](/lib/rkplat/src/riscv64/mcause.rs) | Rust | 38 | 9 | 5 | 52 |
| [lib/rkplat/src/riscv64/mod.rs](/lib/rkplat/src/riscv64/mod.rs) | Rust | 30 | 10 | 9 | 49 |
| [lib/rkplat/src/riscv64/mstatus.rs](/lib/rkplat/src/riscv64/mstatus.rs) | Rust | 173 | 26 | 13 | 212 |
| [lib/rkplat/src/riscv64/reg.rs](/lib/rkplat/src/riscv64/reg.rs) | Rust | 105 | 14 | 5 | 124 |
| [lib/rkplat/src/riscv64/sbi.rs](/lib/rkplat/src/riscv64/sbi.rs) | Rust | 30 | 5 | 5 | 40 |
| [lib/rkplat/src/riscv64/spinlock.rs](/lib/rkplat/src/riscv64/spinlock.rs) | Rust | 116 | 9 | 14 | 139 |
| [lib/rkplat/src/riscv64/thread.rs](/lib/rkplat/src/riscv64/thread.rs) | Rust | 46 | 25 | 9 | 80 |
| [lib/rkplat/src/riscv64/time.rs](/lib/rkplat/src/riscv64/time.rs) | Rust | 65 | 18 | 16 | 99 |
| [lib/rkring/src/lib.rs](/lib/rkring/src/lib.rs) | Rust | 175 | 76 | 39 | 290 |
| [lib/rksched/src/lib.rs](/lib/rksched/src/lib.rs) | Rust | 34 | 29 | 9 | 72 |
| [lib/rksched/src/sched.rs](/lib/rksched/src/sched.rs) | Rust | 95 | 71 | 23 | 189 |
| [lib/rksched/src/thread.rs](/lib/rksched/src/thread.rs) | Rust | 401 | 120 | 80 | 601 |
| [lib/rksched/src/wait.rs](/lib/rksched/src/wait.rs) | Rust | 95 | 28 | 16 | 139 |
| [lib/rkschedcoop/src/lib.rs](/lib/rkschedcoop/src/lib.rs) | Rust | 305 | 77 | 34 | 416 |
| [lib/rkschedpreem/src/lib.rs](/lib/rkschedpreem/src/lib.rs) | Rust | 67 | 27 | 10 | 104 |
| [lib/rksignal/src/lib.rs](/lib/rksignal/src/lib.rs) | Rust | 5 | 26 | 4 | 35 |
| [lib/rksignal/src/signal.rs](/lib/rksignal/src/signal.rs) | Rust | 45 | 31 | 14 | 90 |
| [lib/rksignal/src/sigset.rs](/lib/rksignal/src/sigset.rs) | Rust | 45 | 17 | 15 | 77 |
| [makefile](/makefile) | Makefile | 31 | 32 | 14 | 77 |
| [makefiles/report.mk](/makefiles/report.mk) | Makefile | 12 | 31 | 11 | 54 |
| [makefiles/test.mk.sh](/makefiles/test.mk.sh) | Shell Script | 61 | 5 | 11 | 77 |
| [reference/Unikraft helloworld/make0.log](/reference/Unikraft%20helloworld/make0.log) | Log | 96 | 0 | 1 | 97 |
| [reference/Unikraft helloworld/make1.log](/reference/Unikraft%20helloworld/make1.log) | Log | 118 | 0 | 5 | 123 |
| [reference/Unikraft helloworld/make2.log](/reference/Unikraft%20helloworld/make2.log) | Log | 107 | 0 | 1 | 108 |
| [reference/qemu-benchmark.log](/reference/qemu-benchmark.log) | Log | 206 | 0 | 25 | 231 |
| [report/0_timeline/README.md](/report/0_timeline/README.md) | Markdown | 249 | 0 | 133 | 382 |
| [report/1_pre-research/README.md](/report/1_pre-research/README.md) | Markdown | 710 | 0 | 364 | 1,074 |
| [report/2_research/ClickOS等调研.md](/report/2_research/ClickOS%E7%AD%89%E8%B0%83%E7%A0%94.md) | Markdown | 45 | 0 | 40 | 85 |
| [report/2_research/README.md](/report/2_research/README.md) | Markdown | 117 | 0 | 83 | 200 |
| [report/2_research/RustyHermit等调研.md](/report/2_research/RustyHermit%E7%AD%89%E8%B0%83%E7%A0%94.md) | Markdown | 77 | 0 | 40 | 117 |
| [report/2_research/Rust语言的优越性.md](/report/2_research/Rust%E8%AF%AD%E8%A8%80%E7%9A%84%E4%BC%98%E8%B6%8A%E6%80%A7.md) | Markdown | 44 | 0 | 42 | 86 |
| [report/2_research/Unikernel的安全性问题.md](/report/2_research/Unikernel%E7%9A%84%E5%AE%89%E5%85%A8%E6%80%A7%E9%97%AE%E9%A2%98.md) | Markdown | 36 | 0 | 7 | 43 |
| [report/2_research/research-report.tex](/report/2_research/research-report.tex) | LaTeX | 967 | 55 | 205 | 1,227 |
| [report/2_research/unikernel往年概述.md](/report/2_research/unikernel%E5%BE%80%E5%B9%B4%E6%A6%82%E8%BF%B0.md) | Markdown | 114 | 0 | 125 | 239 |
| [report/3_feasibility/README.md](/report/3_feasibility/README.md) | Markdown | 2 | 0 | 3 | 5 |
| [report/3_feasibility/Unikraft源代码分析.md](/report/3_feasibility/Unikraft%E6%BA%90%E4%BB%A3%E7%A0%81%E5%88%86%E6%9E%90.md) | Markdown | 174 | 1 | 43 | 218 |
| [report/3_feasibility/assets/Runikraft-architecture.svg](/report/3_feasibility/assets/Runikraft-architecture.svg) | XML | 1,182 | 1 | 2 | 1,185 |
| [report/3_feasibility/feasibility-report.bib](/report/3_feasibility/feasibility-report.bib) | BibTeX | 127 | 0 | 3 | 130 |
| [report/3_feasibility/feasibility-report.tex](/report/3_feasibility/feasibility-report.tex) | LaTeX | 1,090 | 16 | 197 | 1,303 |
| [report/3_feasibility/therotical_basis.md](/report/3_feasibility/therotical_basis.md) | Markdown | 34 | 0 | 26 | 60 |
| [report/3_feasibility/技术依据-RISCV特权指令与KVM.md](/report/3_feasibility/%E6%8A%80%E6%9C%AF%E4%BE%9D%E6%8D%AE-RISCV%E7%89%B9%E6%9D%83%E6%8C%87%E4%BB%A4%E4%B8%8EKVM.md) | Markdown | 122 | 0 | 70 | 192 |
| [report/3_feasibility/技术依据-Rust编写操作系统的可行性分析.md](/report/3_feasibility/%E6%8A%80%E6%9C%AF%E4%BE%9D%E6%8D%AE-Rust%E7%BC%96%E5%86%99%E6%93%8D%E4%BD%9C%E7%B3%BB%E7%BB%9F%E7%9A%84%E5%8F%AF%E8%A1%8C%E6%80%A7%E5%88%86%E6%9E%90.md) | Markdown | 103 | 0 | 58 | 161 |
| [report/4_midterm/README.md](/report/4_midterm/README.md) | Markdown | 21 | 0 | 15 | 36 |
| [report/COPYING.images.md](/report/COPYING.images.md) | Markdown | 46 | 0 | 3 | 49 |
| [report/runikraft-report.cls](/report/runikraft-report.cls) | TeX | 89 | 16 | 13 | 118 |
| [sudoku/src/main.rs](/sudoku/src/main.rs) | Rust | 152 | 39 | 37 | 228 |
| [support/transform_logo.py](/support/transform_logo.py) | Python | 8 | 1 | 3 | 12 |
| [test/alloc_buddy0/src/main.rs](/test/alloc_buddy0/src/main.rs) | Rust | 67 | 5 | 8 | 80 |
| [test/alloc_buddy1/src/main.rs](/test/alloc_buddy1/src/main.rs) | Rust | 58 | 3 | 7 | 68 |
| [test/global_alloc0/src/main.rs](/test/global_alloc0/src/main.rs) | Rust | 69 | 1 | 26 | 96 |
| [test/list0/src/main.rs](/test/list0/src/main.rs) | Rust | 103 | 6 | 9 | 118 |
| [test/lock0/src/main.rs](/test/lock0/src/main.rs) | Rust | 62 | 0 | 11 | 73 |
| [test/rkgpu0/src/main.rs](/test/rkgpu0/src/main.rs) | Rust | 18 | 4 | 4 | 26 |
| [test/rkplat_time0/src/main.rs](/test/rkplat_time0/src/main.rs) | Rust | 14 | 0 | 4 | 18 |
| [test/sched_coop0/src/main.rs](/test/sched_coop0/src/main.rs) | Rust | 83 | 0 | 10 | 93 |
| [test/sched_coop1/src/main.rs](/test/sched_coop1/src/main.rs) | Rust | 92 | 0 | 12 | 104 |
| [test/slist0/src/main.rs](/test/slist0/src/main.rs) | Rust | 77 | 6 | 8 | 91 |
| [test/stailq0/src/main.rs](/test/stailq0/src/main.rs) | Rust | 92 | 7 | 9 | 108 |
| [test/tailq0/src/main.rs](/test/tailq0/src/main.rs) | Rust | 127 | 8 | 11 | 146 |

[Summary](results.md) / Details / [Diff Summary](diff.md) / [Diff Details](diff-details.md)