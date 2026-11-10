# WhatsApp Desktop Native (Pure Rust)

A ultra-lightweight, high-performance standalone Windows desktop client for WhatsApp Web built entirely in **Rust** using `tao` and `wry` (native Edge WebView2 runtime).

## ✨ Features

- **True Single Standalone `.exe`**: Requires **NO** external DLLs, Python scripts, batch files, or extra folders.
- **Ultra-Low Resource Consumption**: Uses only **~30–40 MB of RAM** (unlike Electron apps consuming 350+ MB).
- **Embedded Icons**: Multi-resolution official WhatsApp ICO embedded directly in the PE binary header and taskbar runtime.
- **Window State Preservation**: Automatically remembers position, size, and maximize state across restarts.
- **Fast Startup**: Launches almost instantaneously with hardware acceleration.

## 🚀 Usage

Download the single executable `WhatsApp.exe` from [Releases](https://github.com/) (or build from source) and run it anywhere:

```cmd
WhatsApp.exe
```

## 🛠️ Building from Source

### Prerequisites
- [Rust & Cargo](https://rustup.rs/) (stable channel)
- Windows 10 / 11 with Microsoft Edge WebView2 runtime (pre-installed by default on Windows)
- MinGW `windres` (for embedding the `.rc` icon)

### Build Command

```cmd
cargo build --release
```

The optimized, standalone executable will be located at:
```
target/release/whatsapp-rust.exe
```
Rename it to `WhatsApp.exe` and place it anywhere.
