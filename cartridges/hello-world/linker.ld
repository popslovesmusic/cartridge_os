/* Linker script for Hello World Cartridge */

ENTRY(_start)

SECTIONS
{
    /* Cartridge loaded at address determined by switcher */
    /* For now, use 4MB as placeholder */
    . = 4M;

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
