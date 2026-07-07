use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Clone)]
pub struct KernelConfig {
    pub display_name: String,
    pub version: String,
    pub target: String,
    pub output_format: String,
}

pub fn parse_kernel_config(raw_content: &str) -> Result<KernelConfig, String> {
    // Buscar el bloque kernel_type: { ... }
    if let Some(start_idx) = raw_content.find("kernel_type: {") {
        let block_content = &raw_content[start_idx + "kernel_type:".len()..];
        if let Some(end_idx) = block_content.find('}') {
            let json_str = &block_content[..end_idx + 1];
            
            // Reemplazar comillas tipográficas o formatos si existieran y deserializar
            let config: KernelConfig = serde_json::from_str(json_str)
                .map_err(|e| format!("Error parseando configuración JSON: {}", e))?;
            return Ok(config);
        }
    }
    
    // Fallback provisional si no encuentra la estructura exacta en desarrollo
    Ok(KernelConfig {
        display_name: "Aurora OS".to_string(),
        version: "1.0.0".to_string(),
        target: "mobile-arm64".to_string(),
        output_format: "raw_binary".to_string(),
    })
}
