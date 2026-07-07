.section ".text.boot"
.global _start

_start:
    /* Asegurar que solo el núcleo principal (Core 0) ejecuta el inicio */
    mrs     x0, mpidr_el1
    and     x0, x0, #0xFF
    cbz     x0, .Lcpu_master
    
.Lcpu_halt:
    /* Apagar otros núcleos para evitar condiciones de carrera */
    wfe
    b       .Lcpu_halt

.Lcpu_master:
    /* Limpiar la sección .bss (memoria no inicializada) */
    ldr     x0, =__bss_start
    ldr     x1, =__bss_end
.Lbss_loop:
    cmp     x0, x1
    b.hs    .Lbss_done
    str     xzr, [x0], #8
    b       .Lbss_loop

.Lbss_done:
    /* Establecer el puntero de pila (Stack Pointer) antes de saltar a Rust */
    ldr     x0, =_start
    mov     sp, x0

    /* Saltar al punto de entrada del Kernel en Rust */
    bl      kernel_main

    /* Bucle de seguridad por si el kernel llega a retornar */
1:  wfe
    b       1b
