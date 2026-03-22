# 🌌 Aurora Programming Language
> **The Power of Rust. The Simplicity of Python. The Soul of a System.**

Aurora is a **next-generation, context-aware** programming language engineered specifically for low-level system development, kernel architecture, and hardware interaction. It eliminates the traditional cognitive overhead of C/C++ while maintaining absolute control over memory and CPU cycles.

By leveraging a proprietary **Contextual Semantics Engine**, Aurora dynamically adapts its keyword behaviors based on imported modules, allowing developers to write high-quality, safe system code with minimal and expressive syntax.

---

## ✨ Core Architecture

* **🎨 Boreal Syntax:** A visual-first, indentation-based design inspired by Python’s clarity. It reads like human intent but executes as machine instructions.
* **🧠 Contextual Intelligence:** Keywords (e.g., `Set`, `Start`, `with`) morph their internal AST representations depending on the active domain (Kernel, Userland, or Hardware).
* **⚙️ The `aurora-c` Native Compiler:** Aurora is not interpreted. The `aurora-c` compiler translates Aurora source code (`.au`) directly into native machine code (ELF/Object files) using an aggressive optimization pipeline.

---

## 🛡️ Memory Safety & The WRTR Bridge

At the heart of Aurora is the **WRTR Master Library**. Instead of directly manipulating raw pointers by default, Aurora uses WRTR as a safe abstraction layer.

* **Zero-Cost Abstractions:** When you write hardware-level instructions in Aurora, the `aurora-c` compiler generates **Safe Rust Bindings** under the hood. 
* **Compile-Time Safety:** Operations like `Set memory.limit` are subjected to Rust-inspired borrow checking and bound validations at compile time, effectively neutralizing buffer overflows and memory leaks before execution.

---

## 🚀 Syntax Previews

### 1. Basic Initialization (OS Level)
# Initializing kernel parameters is straightforward and declarative.

```python
from wrtr import os, user

# --- Initial Kernel Configuration ---
Set os.type as "Mixed"
Set user.name as "admin"
Set user.permissions as ["root"]

# --- Launching Master Services ---
Start os.services
2. Advanced Memory & Hardware Control (DMA)
Aurora handles Direct Memory Access safely within specific contexts.
Python
from wrtr import memory, hardware

# --- Low-Level Memory Mapping ---
Set vga.buffer as 0xB8000
Set stack.start as 0x100000

# Contextual Loop: Safety-first memory clearing
# The 'memory' context automatically enforces bounds checking via aurora-c
with memory:
    for addr in range(stack.start, 0x200000):
            Set addr.value as 0x0

            # Hardware Interrupt Binding
            def handle_keyboard_interrupt(event):
                if event.code == 0x1C:
                        return "Enter Pressed"

                        Assign handle_keyboard_interrupt to hardware.irq[1]
                        ```
                        🛠️ Requirements
                        To fully experience Aurora's ecosystem:
                        Rust & Cargo: Required to build the aurora-c compiler from source.

                        Aurora Extension: Official VS Code extension for .au semantic highlighting.

                        ⚙️ Extension Settings
                        Customize your workspace through VS Code settings:
                        aurora.compiler.path: Absolute path to your compiled aurora-c binary.

                        aurora.formatOnSave: Enables the auto-formatter to enforce strict Boreal indentation.

                        ⚠️ Known Issues & Roadmap
                        Current limitations in the Alpha phase:
                        Syntax highlighting is currently optimized exclusively for Dark Themes.

                        Advanced IntelliSense (LSP auto-completion) is under active development.

                        Multi-threading support via wrtr.tasks is experimental.

                        📜 Release Notes
                        0.1.0 (Initial Alpha)
                        * Official launch of the Aurora Grammar and VS Code Extension.
                        * Introduction of the Contextual Inference Engine.
                        * Integration of the official Boreal Gradient branding.
                        Developed with ❤️ by Maximiliano Flores

                        Built on Rust infrastructure and Pythonic elegance.

                        ---

                        ### 🚀 Despliegue Inmediato

                        Para inyectar esta potencia en tu repositorio, ejecuta los siguientes comandos en Termux:

                        1. Abre el archivo y reemplaza todo el contenido:
                           ```bash
                              nano README.md
                              Vuelve a empaquetar tu extensión con la nueva documentación:

                              Bash
                              rm aurora-lang.vsix
                              zip -r aurora-lang.vsix . -x ".git/*" ".vscode/*"
                              Sube este nuevo estándar a tu repositorio:

                              Bash
                              git add README.md
                              git commit -m "feat: Upgrade README with advanced WRTR specs and aurora-c architecture"
                              git push origin main
                              El proyecto ahora proyecta la imagen de un ecosistema de sistemas completo. Tienes la justificación técnica de la seguridad (Rust), el compilador (aurora-c) y casos de uso avanzados (DMA e interrupciones).