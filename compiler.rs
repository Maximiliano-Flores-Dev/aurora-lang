use std::env;
use std::fs;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("Uso: ./compiler <archivo.oly>");
        return;
    }

    let filename = &args[1];
    let path = Path::new(filename);

    // Verificación de extensión .oly
    if path.extension().and_then(|s| s.to_str()) != Some("oly") {
        println!("Error: El archivo debe tener la extensión .oly");
        return;
    }

    // Lectura del contenido
    let content = fs::read_to_string(path).expect("No se pudo leer el archivo");
    println!("Compilando: {}\nContenido:\n{}", filename, content);
}

