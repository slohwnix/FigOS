# FigOS by slohwnix

FigOS is a minimalist kernel written in **Rust**, designed to run natively on UEFI systems.  
It uses the [`uefi-rs`](https://docs.rs/uefi/latest/uefi/) crate as its base.

---

## Features
- **PS/2 Keyboard**: Full typing support in the CLI.
- **UEFI Boot**: Boots natively on modern hardware.
- **Graphics Backends**: Supports both **UEFI FRAMEBUFFER** and a minimal GPU backend.
- **CLI**: Built-in shell with commands like `fetch`, `clear`, and `say`.

---

## Getting Started

### Prerequisites
- **Rust**: Install via [rust-lang.org](https://www.rust-lang.org/tools/install)
- **Python 3**: Needed for `.iso` generation.
- **Linux (Debian Based) / WSL (for .iso build)**: Install necessary tools:
```bash
sudo apt update && sudo apt install mtools xorriso dosfstools
```
- **QEMU**: Required for running and testing the kernel.
- **OVMF.fd**: Required for running and testing the kernel. ( you need to move the ovmf.fd in the project folder )

> Note: On Windows, WSL is only needed for `.iso` builds. For `.efi`, you only need Rust and QEMU. 

---

### Build `.efi`
Simply run:
```bash
cargo build
```
The compiled EFI binary will be at:
target/x86_64-unknown-uefi/debug/FigOS.efi

---

### Build `.iso`
After building the `.efi` file, run:
```bash
python3 ./make_iso.py
```
---

### Run
With prerequisites installed, you can run FigOS directly:
```bash
cargo run
```
---

### Notes
- The `rust-toolchain.toml` file ensures the correct nightly Rust version and target are automatically set.
- FigOS is experimental and designed for hobbyist OS development.
