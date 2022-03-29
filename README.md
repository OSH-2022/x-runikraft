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

<!--本人水平有限，这段翻译得很烂-->Runikraft is a unikernel written in Rust language that runs on RSIC-V + KVM. Referencing the architecture of  Unikraft<!--reference应该是人的动作，不过中文原文也是这么写的-->, while inheriting the efficiency, customizablility, compatibility and security of Unikraft, it further simplifies the building process of the OS image, adds support to RISC-V and enhances the kernel security via Rust language.

## Directories Description

- `docs`: project reports for submission purpose (despite its name, it does not contain any documentation)
- `references`: references and bibliographies

 ## Usage

## Building

As different parts of this project were written in difference languages, we use `make` as a universal building system. However, you may manually build each part separately without `make`. 

To build everything without reports and documentations, install all the dependencies:

- Rust compiler (`rustc >= 1.57`+`cargo`): `apt install rust-all` or use `rustup`.
- make: `apt install make`.

Then simply run:

```
make
```

Building the reports and documentations requires XeLaTeX (recommend TeX Live 2021+) and some CJK fonts:

- Noto Serif CJK SC Regular+Bold, Noto Sans CJK SC DemiLight+Bold: `apt install fonts-noto-cjk fonts-noto-cjk-extra`.
- SimFang, SimKai: they are available in [this repository](https://github.com/Halfish/lstm-ctc-ocr/tree/master/fonts). (sorry for using non-free fonts, for we hasn't found free alternatives yet)

Then run:

```
make report
make doc
```

## Contribution

## License

## Contact Us

