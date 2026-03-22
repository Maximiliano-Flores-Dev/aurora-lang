# 🌌 AURORA: THE SOVEREIGN SYSTEM LANGUAGE
> **The Power of Rust. The Simplicity of Python. The Soul of a System.**

Aurora is a next-generation system programming language engineered for low-level development, kernel architecture, and safe hardware interaction. It eliminates cognitive overhead while maintaining absolute control over CPU cycles and memory.

---

## 🌌 PHILOSOPHY: THE BRIDGE TO THE MACHINE

Aurora rejects the duality between the Human (abstraction) and the Machine (complexity). Through its **Contextual Semantics Engine**, the language adapts its behavior based on the active domain:

* **Boreal Syntax:** A visual-first design based on indentation. It reads like human intent but executes as machine instructions.
* **Contextual Intelligence:** The aurora-c compiler (Rust-based) dynamically redefines AST representations based on imported modules from **WRTR**.
* **Memory Sovereignty:** Full memory control without a Garbage Collector, protected by the WRTR safe abstraction layer.

---

## 🏛️ SYSTEM ARCHITECTURE (COMPILATION PIPELINE)

(Visual representation of the aurora-c pipeline)
[ .au Source ] --> [ Lexer/Parser ] --> [ Context Engine ] --> [ WRTR Safety Bridge ] --> [ CPU/Memory ]

---

## 📜 SYNTAX PREVIEW (STANDARDIZED)

### I. Operating System Configuration
*Initializing kernel parameters with declarative clarity.*

```
from wrtr import os, user

# --- Initial Kernel Configuration ---
Set os.type as "Mixed"
Set user.name as "admin"
Set user.permissions as ["root"]

Start os.services
```

### II. Low-Level Memory Management
*Direct Memory Access (DMA) with explicit access control.*

```
from wrtr import memory

# Temporary lock for DMA with Read/Write permissions
with memory(access="RW"):
    Set 0xB8000 as 0x41 # Writes Byte 'A' to VGA buffer

    # MMU Configuration for the current process thread
    Set process.stack.start as 0x100000
    Set process.stack.size as 0x1000
    ```

    ### III. Hardware & Interrupts
    *Managing I/O ports and ISR (Interrupt Service Routines).*

    ```
    from wrtr import hardware

    # Out-Byte instruction to I/O Port 60h
    Set port.0x60 as 0x01

    # Define an Asynchronous Interrupt Handler
    def handle_keyboard_interrupt(event):
        if event.code == 0x1C:
                return "Enter Pressed"

                # Map the handler to IRQ 1 (Keyboard)
                Set hardware.irq[1] as handle_keyboard_interrupt
                ```

                ---

                ## ⚙️ ENVIRONMENT & SETUP

                1. **Compiler:** aurora-c (Built with Rust & Cargo).
                2. **Extensions:** Aurora Boreal Syntax (.vsix included in /tools).
                3. **Requirements:**
                   * Rust Toolchain (Edition 2021+).
                      * Termux (Android) or Linux Environment.

                      ### Installation (Manual for Alpha):
                      ```
                      cd aurora-compiler
                      cargo build --release
                      cp target/release/aurora-c /usr/local/bin/
                      ```

                      ---

                      ## 📖 TECHNICAL GLOSSARY

                      | Term | Definition |
                      | :--- | :--- |
                      | **Boreal Syntax** | Indentation-based structural logic emphasizing intent. |
                      | **WRTR Library** | Foundation safety layer providing secure Rust bindings. |
                      | **aurora-c** | High-performance Rust-based native compiler. |
                      | **Set (Operator)** | Unified operator for polymorphic assignment and configuration. |
                      | **Context Switch** | Mechanism where from...import redefines keyword semantics. |

                      ---

                      ## 🗺️ ROADMAP 2026

                      * [x] **v0.1.0:** Launch aurora-c Lexer and Syntax Highlighting.
                      * [ ] **v0.2.0:** Full Semantic Analysis and Symbol Table for WRTR modules.
                      * [ ] **v0.5.0:** Native code generation for ARM64/Termux targets.
                      * [ ] **v1.0.0:** First stable build for Sovereign System Architecture.

                      ---

                      **Developed with excellence by Maximiliano Flores**
                      *Built on Rust infrastructure. Inspired by Pythonic elegance.*