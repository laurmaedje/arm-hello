ENTRY(_start)
SECTIONS
{
    . = 0x40000000;
    .text : { *(.text) }
    .data : { *(.data) }
    .bss : { *(.bss) }

    /* Stack */
    . = ALIGN(8);
    . = . + 0x2000;
    stack_top = .;
}