# Getting Started (Beginner Guide)

This guide walks you through running the **Entra Least-Privilege Analyzer** (`elpa`) from scratch, even if you have never used a terminal, Rust, or Git before. Pick the section for your operating system: Windows, Linux, or macOS.

The very first thing you'll run is the built-in **demo mode**, which works immediately with no Entra ID / Azure AD credentials at all.

---

## Windows

### 1. Open a terminal

Right-click the **Start** button and choose **Terminal** (or **Windows PowerShell** on older versions of Windows).

<!-- TODO: Screenshot -->

### 2. Check if Rust is installed

Type the following commands, pressing Enter after each:

```powershell
rustc --version
cargo --version
```

If you see version numbers (e.g. `rustc 1.78.0`), Rust is installed and you can skip to step 3.

If instead you see something like `'rustc' is not recognized as an internal or external command`, Rust is **not installed** (or not on your PATH). Install it:

1. Go to [https://rustup.rs](https://rustup.rs)
2. Download `rustup-init.exe`
3. Run it and follow the on-screen prompts (the default options are fine)
4. Close and reopen your terminal, then re-run `rustc --version` and `cargo --version` to confirm

### 3. Get the code

**Easiest way (no Git required):**

1. Open [https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer](https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer)
2. Click the green **Code** button
3. Click **Download ZIP**
4. Extract the ZIP file somewhere convenient, e.g. `C:\Tools\entra-least-privilege-analyzer`
5. In your terminal, navigate into the extracted folder, for example:

```powershell
cd C:\Tools\entra-least-privilege-analyzer
```

**Alternative (if you already have Git installed):**

```powershell
git clone https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer.git
cd entra-least-privilege-analyzer
```

### 4. Build the tool

```powershell
cargo build --release
```

This downloads dependencies and compiles the program. It can take a few minutes the first time.

### 5. Run it

Try the demo first, no credentials needed, it runs against a built-in synthetic tenant:

```powershell
.\target\release\elpa.exe demo
```

Once you have a real Entra ID app registration set up (see the [README](README.md) for the required Graph API permissions), you can run a real analysis:

```powershell
.\target\release\elpa.exe analyze
```

### What to expect

The tool prints a table to your terminal showing the number of users, role assignments, and findings, followed by a list of findings with a severity (`CRITICAL`, `HIGH`, `MEDIUM`, `LOW`), a title describing the issue (e.g. an over-privileged account or a PIM gap), and how many accounts are affected. You can also export results as Markdown or JSON, see the [README](README.md) Quick Start section for the exact commands.

### Troubleshooting

| Problem | Cause | Fix |
|---|---|---|
| `'rustc' is not recognized...` even after installing Rust | Terminal was opened before Rust was added to PATH | Close and reopen the terminal, then try again |
| `cargo build --release` fails with linker errors | Missing C++ Build Tools required by Rust on Windows | Install the [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (select "Desktop development with C++"), then retry |
| `elpa analyze` hangs or fails to connect | Network/proxy/firewall blocking access to Microsoft Graph | Make sure your machine can reach `login.microsoftonline.com` and `graph.microsoft.com` (check corporate proxy/firewall rules) |

---

## Linux

### 1. Open a terminal

The exact steps depend on your desktop environment. On most distributions, search for "Terminal" in your application menu (e.g. GNOME Activities, KDE application launcher), or use the keyboard shortcut common to your distro (often `Ctrl+Alt+T`).

### 2. Check if Rust is installed

```bash
rustc --version
cargo --version
```

If you see version numbers, Rust is installed, skip to step 3.

If you see `command not found: rustc`, Rust is not installed. Install it with the official installer:

1. Go to [https://rustup.rs](https://rustup.rs)
2. Run the curl one-liner shown on that page, typically:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Follow the prompts (default options are fine)
4. Close and reopen your terminal (or run `source $HOME/.cargo/env`), then re-run `rustc --version` and `cargo --version` to confirm

### 3. Get the code

**Easiest way (no Git required):**

1. Open [https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer](https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer)
2. Click the green **Code** button
3. Click **Download ZIP**
4. Extract the ZIP file, then navigate into it in your terminal:

```bash
cd ~/Downloads/entra-least-privilege-analyzer-main
```

**Alternative (if you already have Git installed):**

```bash
git clone https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer.git
cd entra-least-privilege-analyzer
```

### 4. Build the tool

```bash
cargo build --release
```

### 5. Run it

Try the demo first, no credentials needed:

```bash
./target/release/elpa demo
```

Once you have a real Entra ID app registration set up (see the [README](README.md) for the required Graph API permissions):

```bash
./target/release/elpa analyze
```

### What to expect

The tool prints a table to your terminal showing the number of users, role assignments, and findings, followed by a list of findings with a severity (`CRITICAL`, `HIGH`, `MEDIUM`, `LOW`), a title describing the issue, and how many accounts are affected. You can also export results as Markdown or JSON, see the [README](README.md) Quick Start section for the exact commands.

### Troubleshooting

| Problem | Cause | Fix |
|---|---|---|
| `command not found: rustc` even after installing Rust | Shell wasn't reloaded after install | Close and reopen the terminal, or run `source $HOME/.cargo/env` |
| `cargo build --release` fails with linker errors | Missing a C compiler/linker (`cc`) | Install your distro's build essentials package, e.g. `sudo apt install build-essential` on Debian/Ubuntu |
| `elpa analyze` hangs or fails to connect | Network/proxy/firewall blocking access to Microsoft Graph | Make sure your machine can reach `login.microsoftonline.com` and `graph.microsoft.com` (check proxy/firewall rules) |

---

## macOS

### 1. Open a terminal

Press `Cmd+Space` to open Spotlight, type `Terminal`, and press Enter.

### 2. Check if Rust is installed

```bash
rustc --version
cargo --version
```

If you see version numbers, Rust is installed, skip to step 3.

If you see `command not found: rustc`, Rust is not installed. Install it:

1. Go to [https://rustup.rs](https://rustup.rs)
2. Run the curl one-liner shown on that page, typically:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

3. Follow the prompts (default options are fine)
4. Close and reopen your terminal (or run `source $HOME/.cargo/env`), then re-run `rustc --version` and `cargo --version` to confirm

### 3. Get the code

**Easiest way (no Git required):**

1. Open [https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer](https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer)
2. Click the green **Code** button
3. Click **Download ZIP**
4. Extract the ZIP file, then navigate into it in your terminal:

```bash
cd ~/Downloads/entra-least-privilege-analyzer-main
```

**Alternative (if you already have Git installed):**

```bash
git clone https://github.com/9t29zhmwdh-coder/entra-least-privilege-analyzer.git
cd entra-least-privilege-analyzer
```

### 4. Build the tool

```bash
cargo build --release
```

### 5. Run it

Try the demo first, no credentials needed:

```bash
./target/release/elpa demo
```

Once you have a real Entra ID app registration set up (see the [README](README.md) for the required Graph API permissions):

```bash
./target/release/elpa analyze
```

### What to expect

The tool prints a table to your terminal showing the number of users, role assignments, and findings, followed by a list of findings with a severity (`CRITICAL`, `HIGH`, `MEDIUM`, `LOW`), a title describing the issue, and how many accounts are affected. You can also export results as Markdown or JSON, see the [README](README.md) Quick Start section for the exact commands.

### Troubleshooting

| Problem | Cause | Fix |
|---|---|---|
| `command not found: rustc` even after installing Rust | Shell wasn't reloaded after install | Close and reopen the terminal, or run `source $HOME/.cargo/env` |
| `cargo build --release` fails with linker errors | Missing Xcode Command Line Tools | Run `xcode-select --install` and follow the prompts, then retry |
| `elpa analyze` hangs or fails to connect | Network/proxy/firewall blocking access to Microsoft Graph | Make sure your machine can reach `login.microsoftonline.com` and `graph.microsoft.com` (check proxy/firewall rules) |

---

## Next steps

Once `elpa demo` works, head back to the [README](README.md) for how to register an Entra ID application and run a real analysis against your own tenant.
