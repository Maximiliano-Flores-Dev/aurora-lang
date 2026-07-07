use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SecurityConfig {
    pub telemetry: bool,
    pub privacy_mode: String,
    pub sandbox_apps: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KernelConfig {
    pub display_name: String,
    pub version: String,
    pub target: String,
    pub output_format: String,
    pub security: SecurityConfig,
}
