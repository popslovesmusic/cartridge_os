/* Linker script for Cartridge OS kernel */

ENTRY(_start)

SECTIONS
{
    /* Kernel loaded at 1MB physical address */
    . = 1M;

    .boot :
    {
        /* Entry point first */
        *(.multiboot)
        *(.text.boot)
    }

    .text :
    {
        *(.text .text.*)
    }

    .rodata :
    {
        *(.rodata .rodata.*)
    }

    .data :
    {
        *(.data .data.*)
    }

    .bss :
    {
        *(COMMON)
        *(.bss .bss.*)
    }

    /DISCARD/ :
    {
        *(.eh_frame)
        *(.comment)
    }
}
