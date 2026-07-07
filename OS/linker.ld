ENTRY(_start)

SECTIONS
{
    /* Dirección típica de carga para kernels ARM64 (por ejemplo, en QEMU o boards de desarrollo) */
    . = 0x40080000;

    .text : {
        *(.text.boot) /* Asegura que el código de arranque vaya al inicio absoluto */
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        ALIGN(8);
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    }

    /DISCARD/ : {
        *(.comment)
        *(.gnu.attributes)
        *(.eh_frame)
    }
}
