OUTPUT_ARCH(riscv)
ENTRY(__runikraft_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
        _rk_thread_inittab_start = .
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
        _rk_thread_inittab_end = .
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4K);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }

    . = ALIGN(4K);
    ebss = .;
    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame)
    }
} 
