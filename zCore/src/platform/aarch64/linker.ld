ENTRY(_start)
BASE_ADDRESS = 0xffff000040080000;

SECTIONS
{
    . = BASE_ADDRESS;
    skernel = .;

    .text ALIGN(4K): {
        stext = .;
        *(.text.entry)
        *(.text .text.*)
        . = ALIGN(4K);
        etext = .;
    }

    .rodata ALIGN(4K): {
        srodata = .;
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    .kcounter.desc ALIGN(4K):
    {
        PROVIDE_HIDDEN(kcounters_desc_vmo_start = .);
        KEEP(*(.kcounter.desc.header))
        QUAD(kcounters_desc_end - kcounters_desc_start);
        ASSERT(. - kcounters_desc_vmo_start == 24, "wrong size of the kcounter descriptor VMO header");

        PROVIDE_HIDDEN(kcounters_desc_start = .);
        KEEP(*(SORT_BY_NAME(.kcounter.desc.*)))
        PROVIDE_HIDDEN(kcounters_desc_end = .);
        erodata = .;
    } :rodata

    .data ALIGN(4K): {
        sdata = .;
        *(.data .data.*)
        *(.sdata .sdata.*)
        . = ALIGN(4K);
        edata = .;
    }

    .bss ALIGN(4K): {
        boot_stack = .;
        *(.bss.stack)
        . = ALIGN(4K);
        boot_stack_top = .;

        sbss = .;
        PROVIDE_HIDDEN(kcounters_arena_start = .);
        KEEP(*(SORT_BY_NAME(.bss.kcounter.*)))
        PROVIDE_HIDDEN(kcounters_arena_end = .);
        ASSERT(kcounters_arena_end - kcounters_arena_start == (kcounters_desc_end - kcounters_desc_start) * 8 / 64,
              "kcounters_arena size mismatch");
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        . = ALIGN(4K);
        ebss = .;
    }

    ekernel = .;

    /DISCARD/ : {
        *(.eh_frame) *(.eh_frame_hdr)
    }
}