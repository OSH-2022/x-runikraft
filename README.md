# ![Runikraft_logo](./doc/assets/Runikraft_logo.svg)

USTC 011705 (OSH) Course Project of Runikraft Group

**Attention**: This repository is achieved because the course has ended.

## Group Members

(Unicode codepoint order)
- [吴骏东](https://github.com/wintermelon008)
- [张子辰](https://github.com/WCIofQMandRA)
- [蓝俊玮](https://github.com/Lan13)
- [郭耸霄](https://github.com/gsxgoldenlegendary)
- [陈建绿](https://github.com/hanhainebula)

## Project Summary

Runikraft is a unikernel written in Rust language that runs on RISC-V architecture + QEMU platform. It is based on Unikraft implemented in C language; while inheriting the efficiency, customizability, good-compatibility and security of Unikraft, it further simplifies the building process of the OS image, adds support to RISC-V and enhances the kernel security via Rust language.

[Timeline](https://osh-2022.github.io/x-runikraft/report/0_timeline/)　　　　　[Research report](https://osh-2022.github.io/x-runikraft/report/2_research/)　　　　　[Feasibility report](https://osh-2022.github.io/x-runikraft/report/3_feasibility/)　　　　　[Midterm report](https://osh-2022.github.io/x-runikraft/report/4_midterm)

## Directories Description

- `report`: project reports for submission purpose
- `reference`: references and bibliographies
- `core`: core component of Runikraft
- `lib`: micro-libraries

## Getting Started

### Quick Guideline

For Ubuntu 22.04 users, the following commands will download, build and test Ruikraft:

```shell
sudo apt install git curl crossbuild-essential-riscv64 qemu-system-misc qemu-system-gui libncurses-dev bison flex socat
curl https://sh.rustup.rs | sh
source ~/.cargo/env
rustup target add riscv64gc-unknown-none-elf
git clone --recursive https://github.com/OSH-2022/x-runikraft.git
cd x-runikraft
cargo fetch
make  # -j is not neccessary since make is just a wrapper to cargo
```

### Building Runikraft

As different parts of this project were written in different languages, we use `make` as a universal building system. However, you may manually build each part separately without `make`. 

To build everything without reports, install all the dependencies:

- Rust compiler (`rustc >= 1.59`+`cargo`): you must install it via `rustup`, since rustc in distribution's software source usually does not support riscv64 target.
- C compiler and binutils: any version of riscv64 cross-compile GCC should works.
- QEMU (`qemu-system-riscv64 >= 6.2.0`): we did not test QEMU 5, but we known that QEMU 4 does not support RTC (real-time clock).

Then run:

```shell
make
```

Building the reports requires XeLaTeX (recommend TeX Live 2021+) and some CJK fonts:

- Noto Serif CJK SC Regular+Bold, Noto Sans CJK SC DemiLight+Bold: `apt install fonts-noto-cjk fonts-noto-cjk-extra`, or download from [here](https://mirrors.ustc.edu.cn/ubuntu/pool/main/f/fonts-noto-cjk/fonts-noto-cjk-extra_20220127%2Brepack1-1_all.deb) and [here](https://mirrors.ustc.edu.cn/ubuntu/pool/main/f/fonts-noto-cjk/fonts-noto-cjk_20220127%2Brepack1-1_all.deb).
- SimFang, SimKai: they are available in [this repository](https://github.com/Halfish/lstm-ctc-ocr/tree/master/fonts). (sorry for using non-free fonts, for we haven't found free alternatives yet)

Then run:

```shell
make report
```

## Contributing to Runikraft

This is only a course project, which will no longer be developed or maintained after this semester. Therefore, directly contributing to this repository may not be accepted. However, if you are interested in Runikraft and want to develop a derived work of it, please do not directly fork this repository as it contains many unrelated files like reports and references. Instead, just create a new repository and upload necessary files in this repository. 

## License

Most of Runikraft is distributed under the BSD-3-Clause license; see `LICENSE` for details. Each source file should identify its license. Source code files (including but not limited to `text/x-csrc`, `text/x-chdr`, `text/x-c++src`, `test/x-c++hdr`, `text/rust`, `text/x-makefile`, etc.) that do not identify a specific license are covered by the BSD-3-Clause license. 

The documentations and reports of Runikraft are distributed under the CC-BY-4.0 license; see `report/LICENSE` for details. Document files (including but not limited to `text/markdown`, `text/x-tex`, `text/x-log`, etc.) files that do not identify a specific license are implicitly licensed under the CC-BY-4.0. 

The copyright condition of image files (including but not limited to `image/jpeg`, `image/png`, etc.) are complex, because most of them are not our own works but are extracted from bibliographies. Most of images in our reports are *NOT* free components, so commercial publication of documents in the `report` directory should be performed carefully.  The licenses for images are listed in  `report/COPYING.images.md`. 
