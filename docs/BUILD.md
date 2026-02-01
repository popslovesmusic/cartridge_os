# Building Cartridge OS

This document describes how to build the Cartridge OS components on both Windows and Linux.

## Prerequisites

### Windows (Development)
- Rust toolchain (nightly)
- Git

### Linux/WSL2 (Building & Testing)
- Rust toolchain (nightly)
- QEMU (`qemu-system-x86_64`)
- OVMF (UEFI firmware for QEMU)
- Build tools (`build-essential`)

---

## Development Workflow (Option A: Hybrid)

### 1. Initial Setup on Windows

```powershell
# Navigate to project directory
cd E:\OS

# Install Rust (if not already installed)
# Download from: https://rustup.rs/

# Install required toolchains
rustup toolchain install nightly
rustup component add rust-src rustfmt clippy
rustup target add x86_64-unknown-uefi
```

### 2. Build Tooling on Windows

You can develop and build the host tooling directly on Windows:

```powershell
# Build packager and verifier
cargo build --release --package cartridge-packager --package cartridge-verifier

# Run packager
.\target\release\cartridge-packager.exe --help

# Run verifier
.\target\release\cartridge-verifier.exe --help
```

### 3. Transfer to WSL2 for Kernel/Bootloader Building

#### Enable WSL2 (if not already enabled)

```powershell
# Run as Administrator
wsl --install
wsl --set-default-version 2

# Install Ubuntu
wsl --install -d Ubuntu
```

#### Access Project from WSL2

```bash
# From WSL2 terminal
cd /mnt/e/OS

# Install dependencies
sudo apt update
sudo apt install -y build-essential qemu-system-x86 ovmf curl

# Install Rust in WSL2
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install nightly and components
rustup toolchain install nightly
rustup component add rust-src rustfmt clippy
rustup target add x86_64-unknown-uefi
```

### 4. Build Kernel and Bootloader (WSL2)

```bash
# Build kernel
cargo build --release --package cartridge-kernel --target x86_64-unknown-none

# Build bootloader
cargo build --release --package cartridge-bootloader --target x86_64-unknown-uefi

# Build switcher
cargo build --release --package cartridge-switcher --target x86_64-unknown-none
```

---

## Testing in QEMU (WSL2)

### Create a Bootable Image

```bash
# Install required tools
sudo apt install -y mtools dosfstools

# Create boot image (script to be added)
./scripts/create-image.sh
```

### Run in QEMU

```bash
# Boot the image
qemu-system-x86_64 \
  -bios /usr/share/ovmf/OVMF.fd \
  -drive format=raw,file=boot.img \
  -m 512M \
  -serial stdio
```

---

## Directory Structure

```
E:\OS\
├── kernel/          # Microkernel (bare-metal)
├── bootloader/      # UEFI bootloader
├── switcher/        # Cartridge loader (userspace)
├── tooling/
│   ├── packager/    # Artifact packager (host tool)
│   └── verifier/    # Artifact verifier (host tool)
├── common/          # Shared artifact format library
├── drivers/         # Driver modules (future)
├── cartridges/      # Example application cartridges (future)
└── docs/            # Documentation
```

---

## Build Targets

| Component   | Target                  | Platform       |
|-------------|-------------------------|----------------|
| Packager    | x86_64-pc-windows-msvc  | Windows        |
| Verifier    | x86_64-pc-windows-msvc  | Windows        |
| Kernel      | x86_64-unknown-none     | Bare-metal     |
| Bootloader  | x86_64-unknown-uefi     | UEFI           |
| Switcher    | x86_64-unknown-none     | Bare-metal     |

---

## Troubleshooting

### Windows: Linker errors
- Make sure you have Visual Studio Build Tools or MSVC installed
- Run build from "Developer Command Prompt for VS"

### WSL2: Can't access Windows files
- Windows drives are mounted at `/mnt/c/`, `/mnt/e/`, etc.
- Check with `ls /mnt/e/OS`

### QEMU: No OVMF firmware
```bash
# Ubuntu/Debian
sudo apt install ovmf

# Arch Linux
sudo pacman -S edk2-ovmf
```

### Rust nightly features not available
```bash
rustup update nightly
```

---

## Next Steps

1. Implement proper memory allocator in kernel
2. Add syscall interface between kernel and switcher
3. Create example "Hello World" cartridge
4. Add signature verification to bootloader
5. Implement actual IPC channels

See `docs/DEVELOPMENT.md` for detailed implementation roadmap.
