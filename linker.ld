OUTPUT_ARCH(riscv)
ENTRY(__runikraft_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    __rkplat_boot_skernel = .;

    __rkplat_boot_stext = .;
    .text : {
        *(.text.entry) ;
        _rk_thread_inittab_start = . ;
        *(.text.thread_inittab.0)
        *(.text.thread_inittab.1)
        *(.text.thread_inittab.2)
        *(.text.thread_inittab.3)
        *(.text.thread_inittab.4)
        *(.text.thread_inittab.5)
        *(.text.thread_inittab.6)
        *(.text.thread_inittab.7)
        *(.text.thread_inittab.8)
        *(.text.thread_inittab.9)
        _rk_thread_inittab_end = . ;
        *(.text .text.*)
    }

    . = ALIGN(4K);
    __rkplat_boot_etext = .;
    __rkplat_boot_srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    __rkplat_boot_erodata = .;
    __rkplat_boot_sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    __rkplat_boot_edata = .;
    .bss : {
        *(.bss.stack)
        __rkplat_boot_sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    __rkplat_boot_ebss = .;
    __rkplat_boot_ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
} 
