# x-runikraft

USTC 011705 (OSH) Course Project of Runikraft Group

## Group Members

(Unicode codepoint order)
- [吴骏东](https://github.com/wintermelon008)
- [张子辰](https://github.com/WCIofQMandRA)
- [蓝俊玮](https://github.com/Lan13)
- [郭耸霄](https://github.com/gsxgoldenlegendary)
- [陈建绿](https://github.com/hanhainebula)

## Project Summary

Runikraft is a unikernel written in Rust language that runs on RISC-V architecture + QEMU platform. It is based on Unikraft implemented in C language; while inheriting the efficiency, customizability, good-compatibility and security of Unikraft, it further simplifies the building process of the OS image, adds support to RISC-V and enhances the kernel security via Rust language.

## Directories Description

- `report`: project reports for submission purpose
- `reference`: references and bibliographies

 ## Getting Started

### Building Runikraft

As different parts of this project were written in different languages, we use `make` as a universal building system. However, you may manually build each part separately without `make`. 

To build everything without reports and documentations, install all the dependencies:

- Rust compiler (`rustc >= 1.57`+`cargo`): `apt install rust-all` or use `rustup`.
- make: `apt install make`.

Then run:

```
make
```

Building the reports and documentations requires XeLaTeX (recommend TeX Live 2021+) and some CJK fonts:

- Noto Serif CJK SC Regular+Bold, Noto Sans CJK SC DemiLight+Bold: `apt install fonts-noto-cjk fonts-noto-cjk-extra`, or download from [here](https://mirrors.ustc.edu.cn/ubuntu/pool/main/f/fonts-noto-cjk/fonts-noto-cjk-extra_20220127%2Brepack1-1_all.deb) and [here](https://mirrors.ustc.edu.cn/ubuntu/pool/main/f/fonts-noto-cjk/fonts-noto-cjk_20220127%2Brepack1-1_all.deb).
- SimFang, SimKai: they are available in [this repository](https://github.com/Halfish/lstm-ctc-ocr/tree/master/fonts). (sorry for using non-free fonts, for we haven't found free alternatives yet)

Then run:

```
make report
make doc
```

## Contributing to Runikraft

## License

Most part of Runikraft is distributed under the BSD 3-Clause License; see `LICENSE` for details.

The documentations and reports of Runikraft are distributed under the CC-BY-4.0 License; see `report/LICENSE` for details.

## Contact Us

