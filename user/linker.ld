OUTPUT_FORMAT("elf64-littleaarch64")
OUTPUT_ARCH(aarch64)
ENTRY(_start)

SECTIONS
{
    . = 0x400000000000;
    .text : { *(.text) }
    .rodata : { *(.rodata) }
    /DISCARD/ : { *(.comment) *(.note*) *(.eh_frame*) }
}
