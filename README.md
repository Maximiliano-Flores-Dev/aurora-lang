# `aurora-lang`

A structural, privacy-first systems programming language transpiler designed for infrastructure critical applications and **`Aurora OS`**. Built to guarantee digital sovereignty, maximum execution speed, and absolute user hardware ownership.

---

## 1. El Concepto Arquitectónico (The Core Logic)

`aurora-lang` no es solo una capa sintáctica; es un manifiesto de soberanía tecnológica traducido en un compilador e infraestructura de transpilación modular. El lenguaje adopta una sintaxis híbrida de alto nivel inspirada en la legibilidad limpia de Python y la estructuración declarativa de JSON, diseñada para compilar directamente a código nativo de bajo nivel en Rust con restricciones estrictas de `#![no_std]` and `#![no_main]`.

### Los Pilares de Diseño:
* **Velocidad Extrema:** Rendimiento nativo sin recolector de basura (*Garbage Collector*). Minimización drástica de latencia a través de la orquestación directa del toolchain local (`rustc`).
* **Privacidad Absoluta:** Procesamiento local prioritario. Blindaje nativo de datos y rechazo explícito a telemetría corporativa o teleobservación remota en segundo plano.
* **Control Total:** El usuario es el dueño absoluto del stack tecnológico. El compilador y los drivers resultantes actúan como mediadores conscientes del silicio.
* **Activación Consciente (Anti Always-On Listening):** Integración nativa de hooks de hardware para interrupciones físicas (como la activación del agente inteligente `Claw` tras una pulsación física continua de 3.5s), eliminando la vigilancia pasiva de micrófonos siempre activos.

---

## 2. Flujo del Pipeline del Compilador (0 -> 1)

El motor actual se divide en tres fases críticas automatizadas de ejecución local:

1.  **Parser Éxito (Metadata Extraction):** El núcleo extrae el bloque de configuración declarativo (`kernel_type`) para validar las políticas de seguridad, el formato de salida y el target específico (ej. `mobile-arm64`).
2.  **Codegen Inteligente Multilínea:** El motor procesa el bloque `on_boot` interpretando bloques multilínea complexes. Traduce de manera limpia los namespaces lógicos (`OS.display` -> `OS::display::`), inyecta un módulo de abstracción aislado (*Mock Core Library*), limpia los argumentos nombrados del estilo Python (`pin=4` -> `4`) y asegura la terminación formal de sentencias de Rust (`;\n`).
3.  **El Orquestador:** Invoca de manera nativa y local el compilador de bajo nivel mediante un aislamiento estricto, aplicando flags críticas para evitar la duplicación de símbolos en sistemas host y empaquetar el árbol de directorios directamente en imágenes listas para flashear utilizando `xorriso`.

---

## 3. Especificación de Sintaxis actual (`main.aurora`)

Un archivo base de Aurora OS define metadatos e instrucciones secuenciales de bajo nivel usando una estructura limpia libre de boilerplate:

kernel_type: {
    "display_name": "Aurora OS",
    "version": "1.0.0",
    "target": "mobile-arm64",
    "output_format": "iso",
    "security": {
        "telemetry": false,
        "privacy_mode": "extreme",
        "sandbox_apps": true
    }
}

"on_boot": fn():
    # Inicialización del entorno gráfico local
    OS.display.init_screen()
    OS.display.clear(color="black")
    OS.display.show_log("Aurora OS [V1.0.0] - Cargando modulos de soberania...")

    # Seguridad y Criptografía On-Device
    OS.storage.mount_secure_partition(mount_point="/root", encryption="AES-256-GCM-OnDevice")
    OS.display.show_log("Sistema de archivos local montado con exito.")

    # Activación del planificador en tiempo real
    OS.kernel.start_scheduler()
    OS.display.show_log("Planificador en tiempo real activo.")

    # Blindaje de Hardware y Mecánica de Activación de Claw
    OS.hardware.bind_physical_button(
        pin=4,
        duration_ms=3500,
        trigger_event=OS.agents.claw.awaken
    )
    OS.display.show_log("Modulo 'Claw' asignado al boton fisico (Activacion por pulsacion 3.5s).")

    # Inicialización de Entorno Gráfico Soberano
    OS.display.show_log("Cargando interfaz de usuario vanilla...")
    OS.ui.launch_desktop_environment()

---

## 4. Estructura del Ecosistema Modular

El repositorio se organiza para soportar el crecimiento incremental del sistema operativo y su lenguaje (Ladrillo a Ladrillo):

`aurora-lang/`
├── `Cargo.toml`            # Configuración del ecosistema y dependencias (serde, serde_json)
├── `.gitignore`            # Filtro estricto para evitar telemetría de compilación y binarios pesados
├── `main.aurora`           # Código fuente declarativo y configuración de inicialización del OS
└── `src/`
    └── `main.rs`           # Core del Transpilador, Codegen multilínea y Orquestador del Toolchain

---

## 5. Compilación y Despliegue Local (On-Device via Termux)

Para compilar y empaquetar de manera 100% autónoma y privada desde un entorno móvil nativo, ejecuta:

# Asegurar dependencias del host nativo
pkg install rust xorriso git -y

# Clonar tu repositorio soberano
git clone https://github.com/Maximiliano-Flores-Dev/aurora-lang.git
cd aurora-lang

# Correr el Core del Compilador
cargo run

### Flags Críticas del Enlazador Aplicadas:
Para garantizar que el binario resultante actúe como un Kernel puro `#![no_main]`, el orquestador inyecta los siguientes parámetros a `rustc`:
* `-C panic=abort`: Elimina el peso del mecanismo de stack unwinding.
* `-C link-arg=-nostartfiles`: Desactiva los archivos de inicialización estándar del sistema host, evitando la duplicación del símbolo de entrada global `_start`.

---
*Diseñado bajo la filosofía de la soberanía digital total y control absoluto del hardware.*
