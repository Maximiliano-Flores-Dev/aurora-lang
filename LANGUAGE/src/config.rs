use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct KernelConfig {
    pub display_name: String,
    pub version: String,
    pub target: String,
    pub output_format: String,
}

pub fn parse_kernel_config(raw_content: &str) -> Result<KernelConfig, String> {
    // Valores por defecto soberanos por si no se definen explícitamente
    let mut display_name = "Aurora".to_string();
    let mut version = "1.00.00".to_string();
    let mut target = "mobile-arm64".to_string();
    let mut output_format = "raw_binary".to_string();

    let mut inside_config = false;

    // Procesamos el script .aurora de manera flexible y tolerante a espacios y saltos de línea
    for line in raw_content.lines() {
        let trimmed = line.trim();
        
        if trimmed.starts_with("kernel_type:") {
            inside_config = true;
            continue;
        }

        if inside_config {
            if trimmed.starts_with('}') {
                break; // El bloque de configuración terminó de forma segura
            }

            // Sanitizamos la línea quitando comillas de cadenas y comas residuales
            let clean_line = trimmed.replace('"', "").replace('\'', "").replace(',', "");
            if let Some((key, val)) = clean_line.split_once(':') {
                let k = key.trim();
                let v = val.trim();

                match k {
                    "display_name" => display_name = v.to_string(),
                    "version" => version = v.to_string(),
                    "target" => target = v.to_string(),
                    "output_format" => output_format = v.to_string(),
                    _ => {} // Ignoramos sub-bloques como 'security' por ahora para evitar quiebres
                }
            }
        }
    }

    Ok(KernelConfig {
        display_name,
        version,
        target,
        output_format,
    })
}
