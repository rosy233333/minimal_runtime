OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    stext = .;
    .text : {
        *(.text.entry)
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
    /* _percpu_start = .;
    .percpu : {
        _percpu_load_start = .;
        *(.percpu .percpu.*)
        _percpu_load_end = .;
        . = ALIGN(64);
        _percpu_size_aligned = . - _percpu_load_start;

        . = _percpu_load_start + _percpu_size_aligned * 4;
    }

    . = ALIGN(4K);
    _percpu_end = .; */
    .bss : {
        *(.bss.heap)
        *(.bss.stack0)
        *(.bss.stack1)
        *(.bss.stack2)
        *(.bss.stack3)
        *(.bss.stack4)
        *(.bss.stack5)
        *(.bss.stack6)
        *(.bss.stack7)
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