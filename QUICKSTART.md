# Quick Start Guide

Get up and running with Cartridge OS development in under 10 minutes.

## What You Need

- Windows 10/11
- 10 GB free disk space
- Administrator access (for WSL2 setup)

---

## Step 1: Install Rust (2 minutes)

Open PowerShell and run:

```powershell
# Download and install Rust
# Visit: https://rustup.rs/ and run the installer

# Or use winget (Windows Package Manager)
winget install Rustlang.Rustup

# Close and reopen PowerShell, then:
rustup toolchain install nightly
rustup default nightly
rustup component add rust-src rustfmt clippy
rustup target add x86_64-unknown-uefi
```

---

## Step 2: Build Tooling on Windows (1 minute)

```powershell
cd E:\OS

# Build packager and verifier
cargo build --release --package cartridge-packager --package cartridge-verifier

# Test it works
.\target\release\cartridge-packager.exe --help
```

**You can now create and verify artifacts on Windows!**

---

## Step 3: Install WSL2 for Kernel Building (3 minutes)

Run as Administrator:

```powershell
# Install WSL2 and Ubuntu
wsl --install

# Reboot if prompted
```

After reboot, open Ubuntu from Start Menu and set up username/password.

---

## Step 4: Set Up Build Environment in WSL2 (4 minutes)

```bash
# Update system
sudo apt update && sudo apt upgrade -y

# Install build tools
sudo apt install -y build-essential qemu-system-x86 ovmf curl

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Configure Rust
rustup toolchain install nightly
rustup default nightly
rustup component add rust-src
rustup target add x86_64-unknown-uefi
```

---

## Step 5: Build the Kernel (30 seconds)

```bash
# Navigate to project (from WSL2)
cd /mnt/e/OS

# Build kernel
cargo build --release --package cartridge-kernel --target x86_64-unknown-none

# Build bootloader
cargo build --release --package cartridge-bootloader --target x86_64-unknown-uefi

# Success! You've built the kernel.
```

---

## What You Just Built

```
target/
├── x86_64-unknown-none/
│   └── release/
│       └── cartridge-kernel      # The microkernel
├── x86_64-unknown-uefi/
│   └── release/
│       └── cartridge-bootloader  # UEFI bootloader
└── release/
    ├── cartridge-packager.exe    # Tool to create artifacts
    └── cartridge-verifier.exe    # Tool to verify artifacts
```

---

## Next Steps

### Create Your First Artifact

```powershell
# Create a dummy application (Windows)
echo "Hello from cartridge" > dummy.txt

# Package it as a cartridge
.\target\release\cartridge-packager.exe cartridge `
  --input dummy.txt `
  --output app.cart `
  --capabilities GPU,INPUT

# Verify it
.\target\release\cartridge-verifier.exe app.cart --verbose
```

### Test in QEMU (Coming Soon)

We're still building the bootable image creator script. For now, you have:
- Working tooling to create/verify artifacts
- Compiled kernel and bootloader
- Full development environment ready

---

## Project Structure Overview

```
E:\OS\
├── kernel/          ← Microkernel code (bare-metal Rust)
├── bootloader/      ← UEFI bootloader (verifies kernel)
├── switcher/        ← Cartridge loader (userspace)
├── common/          ← Shared artifact format library
├── tooling/
│   ├── packager/    ← Create sealed artifacts
│   └── verifier/    ← Verify artifact integrity
└── docs/
    ├── BUILD.md            ← Detailed build instructions
    ├── WSL2_SETUP.md       ← WSL2 deep dive
    └── DEVELOPMENT.md      ← Implementation roadmap
```

---

## Troubleshooting

### "rustup: command not found"
Close and reopen your terminal after installing Rust.

### Can't build kernel: "error: no target specified"
Make sure you're using the custom target:
```bash
cargo build --target x86_64-unknown-none
```

### WSL2: "Cannot access /mnt/e/"
Check that your E: drive exists in Windows. Use `/mnt/c/` if project is on C:.

### QEMU not found
```bash
sudo apt install qemu-system-x86
```

---

## Getting Help

- **Read the specs**: `specification-sheet-detailed.md`, `technical-blueprint-detailed.md`
- **Build issues**: See `docs/BUILD.md`
- **WSL2 setup**: See `docs/WSL2_SETUP.md`
- **Development roadmap**: See `docs/DEVELOPMENT.md`
- **Open an issue**: Report problems on the repository

---

## You're Ready!

You now have:
- ✓ Complete development environment (Windows + WSL2)
- ✓ Working Rust toolchain with cross-compilation
- ✓ Built kernel and bootloader
- ✓ Artifact creation and verification tools

Start exploring the code or jump into Phase 1 of the development roadmap!

**Welcome to Cartridge OS development** 🚀
