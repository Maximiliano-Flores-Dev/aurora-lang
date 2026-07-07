# Aurora Lang 
Aurora is a high-level, declarative language and compiler designed for developing sovereign, bare-metal operating systems. It simplifies low-level programming by abstracting hardware interactions into a clean, easy-to-read syntax, which is then transpiled into optimized, `no-std` Rust code for AArch64 targets.

## Key Components

The repository is structured into two main parts: the **language compiler** and the **bare-metal OS foundation**.

-   **`LANGUAGE/`**: The core of the project, this directory contains the Aurora compiler written in Rust. It parses `.aurora` files, transpiles them into low-level Rust code, and compiles the result into a final ELF binary.
-   **`OS/`**: This directory contains the foundational components for the operating system, including the assembly bootloader (`boot.S`), a custom linker script (`linker.ld`), and serves as the output destination for the compiled kernel (`aurora_kernel.elf`).

## How It Works

The Aurora build process automates the creation of a kernel binary from a high-level definition file.

1.  **High-Level Definition**: You define the OS boot logic in an `.aurora` file. This includes kernel metadata and a sequence of commands to run at startup.
2.  **Parsing**: The compiler (`LANGUAGE/src/main.rs`) reads the `.aurora` file. It parses the `kernel_type` configuration block and tokenizes the commands within the `on_boot` function.
3.  **Transpilation (Codegen)**: Each command from the `on_boot` block is converted into a corresponding `no-std` Rust function call. For example, `OS.display.clear()` in Aurora is transpiled to `OS::display::clear()` in Rust. The compiler generates a complete, self-contained `kernel_output.rs` file.
4.  **Final Compilation**: The build pipeline invokes `rustc` with specific flags for bare-metal compilation (`-C opt-level=2`, `--crate-type=bin`). It compiles `kernel_output.rs` into the final `aurora_kernel.elf` binary, which is placed in the `OS/` directory.

## How to Build

To compile the example kernel:

1.  Navigate to the compiler's directory:
    ```bash
    cd LANGUAGE/
    ```

2.  Run the build pipeline with Cargo:
    ```bash
    cargo run
    ```

This single command will perform all the necessary steps: read `../EXAMPLES/boot_test.aurora`, generate the `kernel_output.rs` file in the current directory, and compile it to produce the final `aurora_kernel.elf` binary inside the `/OS` directory.

## Project Structure

```
├── EXAMPLES/
│   └── boot_test.aurora      # Example high-level OS definition
├── LANGUAGE/
│   ├── src/                  # Aurora language compiler source code (Rust)
│   │   ├── parser/           # Tokenizer for .aurora files
│   │   ├── codegen/          # Rust code generation modules
│   │   ├── build_pipeline.rs # Orchestrates the final ELF compilation
│   │   └── main.rs           # Compiler entry point
│   └── Cargo.toml            # Compiler dependencies
└── OS/
    ├── boot.S                # AArch64 assembly bootloader
    ├── linker.ld             # Linker script for memory layout
    └── aurora_kernel.elf     # [Output] The final compiled OS binary
```

## Example: `boot_test.aurora`

The following example demonstrates how to define a simple boot sequence in the Aurora language. It configures kernel metadata and specifies tasks like initializing the screen, mounting a filesystem, and starting the scheduler.

```aurora
From WRITER import OS

kernel_type: {
    "display_name": "Aurora",
    "version": "1.00.00",
    "target": "mobile-arm64",
    "output_format": "raw_binary",
    "security": {
        "telemetry": false,
        "privacy_mode": "strict",
        "sandbox_apps": true
    }
}

"on_boot": fn():
    OS.display.init_screen()
    OS.display.clear(color="black")
    OS.display.show_log("Aurora OS [V1.00.00] - Cargando modulos de soberania...")
    OS.display.draw_boot_pattern()
    OS.storage.mount_secure_partition(mount_point="/root", encryption="AES-256-GCM")
    OS.display.show_log("Sistema de archivos local montado con exito.")
    OS.kernel.start_scheduler()
    OS.display.show_log("Planificador en tiempo real activo.")
    OS.hardware.bind_physical_button(pin=4, duration_ms=3500, trigger_event=OS.agents.claw.awaken)
    OS.display.show_log("Modulo 'Claw' asignado al boton fisico (Activacion por pulsacion 3.5s).")
    OS.display.show_log("Cargando interfaz de usuario vanilla...")
    OS.ui.launch_desktop_environment()
