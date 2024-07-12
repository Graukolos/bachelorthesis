ENTRY(__start);

SECTIONS {
    .text 0x80000: {
        *(.text.__start)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .bss : {
        *(.bss .bss.*)
    }

    .data : {
        *(.data .data.*)
    }
}