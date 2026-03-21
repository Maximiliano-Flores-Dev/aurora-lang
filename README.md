# 🌌 Aurora Programming Language
> **The Power of Rust. The Simplicity of Python. The Soul of a System.**

Aurora is a **next-generation, context-aware** programming language built for low-level system development without the traditional cognitive overhead. By leveraging a **Contextual Semantics** engine, Aurora adapts its behavior based on the imported libraries, allowing for high-quality system code with minimal syntax.

## ✨ Key Features

* **🎨 Boreal Syntax:** A visual-first design inspired by Python’s clarity, optimized for human-readable system instructions.
* **🧠 Contextual Intelligence:** Keyword meanings (like `Set` or `Start`) evolve dynamically based on the active context (Kernel, User, or Hardware).
* **🛡️ Rust-Powered Safety:** All Aurora source code is processed by a dedicated Rust engine, ensuring memory safety and extreme performance.
* **🔌 Native WRTR Integration:** Out-of-the-box support for the WRTR Master Library for system services and memory management.

## 🚀 Syntax Preview

This is how a system initialization file looks in Aurora:

```
from wrtr import os, user

# --- Initial Kernel Configuration ---
Set os.type as "Mixed"
Set user.name as "admin"
Set user.permissions as ["root"]

# --- Launching Master Services ---
Start os.services
```
🛠️ Requirements
To fully experience Aurora, we recommend:

Rust & Cargo: The backend engine that compiles Aurora code.

Aurora Extension: (This extension) for .au file syntax highlighting.

⚙️ Extension Settings
This extension contributes the following settings:

aurora.compiler.path: Define the path to your aurora-compiler binary.

aurora.formatOnSave: Enable auto-formatting to keep indentation perfect.

⚠️ Known Issues
Syntax highlighting is currently optimized for Dark Themes.

Advanced IntelliSense (auto-completion) is under active development.

📜 Release Notes
0.1.0 (Initial Alpha)
Official launch of the Aurora Grammar.

Support for .au file extensions.

Integration of the official Boreal Gradient logo.

Developed with ❤️ by Maximiliano Flores
Built on Rust infrastructure and Pythonic elegance.