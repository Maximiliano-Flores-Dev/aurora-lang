pub mod os {
    pub fn start_services() {
        // Aquí va el código de Rust que toca el hardware
        // Es 'unsafe' por dentro, pero 'safe' para el usuario de Aurora
        unsafe {
            println!("Inicializando controladores de interrupciones...");
            // Código nativo de bajo nivel
        }
    }
}
