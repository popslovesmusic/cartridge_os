/* Linker script for Cartridge OS Switcher */

ENTRY(_start)

SECTIONS
{
    /* Switcher loaded at 2MB physical address (after kernel) */
    . = 2M;

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
