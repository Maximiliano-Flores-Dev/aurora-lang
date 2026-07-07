#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// Punto de entrada llamado por boot.S después de inicializar el procesador
#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    // Aquí es donde el código generado por Aurora (kernel_output.rs)
    // tomará el control físico de los drivers del Framebuffer, memoria, etc.
    
    loop {
        // El procesador se mantiene esperando interrupciones de forma segura
        core::hint::spin_loop();
    }
}

/// Manejador de pánicos obligatorio en entornos #![no_std] (bare-metal)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
