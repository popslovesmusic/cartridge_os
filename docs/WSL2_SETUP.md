# WSL2 Setup Guide for Cartridge OS Development

This guide walks through setting up Windows Subsystem for Linux 2 (WSL2) for building and testing Cartridge OS.

## Why WSL2?

WSL2 provides a real Linux kernel running on Windows, which gives you:
- Native Linux tooling for bootloader/kernel development
- QEMU for testing bootable images
- No VM overhead - direct access to Windows filesystem
- Seamless workflow between Windows development and Linux building

---

## Installation

### 1. Enable WSL2

Open PowerShell as Administrator and run:

```powershell
# Enable WSL feature
wsl --install

# Set WSL2 as default
wsl --set-default-version 2
```

**Reboot your computer if prompted.**

### 2. Install Ubuntu

```powershell
# Install Ubuntu distribution
wsl --install -d Ubuntu

# You'll be prompted to create a username and password
```

### 3. Verify Installation

```powershell
# Check WSL version
wsl --list --verbose

# Should show Ubuntu with VERSION 2
```

---

## Configure Ubuntu for Development

### 1. Update System

```bash
# Enter WSL
wsl

# Update package list
sudo apt update
sudo apt upgrade -y
```

### 2. Install Build Dependencies

```bash
# Core build tools
sudo apt install -y build-essential git curl

# QEMU for testing
sudo apt install -y qemu-system-x86 ovmf

# Image creation tools
sudo apt install -y mtools dosfstools parted
```

### 3. Install Rust Toolchain

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Follow prompts (option 1: standard installation)

# Load Rust environment
source $HOME/.cargo/env

# Install nightly toolchain
rustup toolchain install nightly
rustup default nightly

# Add components
rustup component add rust-src rustfmt clippy

# Add targets
rustup target add x86_64-unknown-uefi
```

### 4. Verify Setup

```bash
# Check Rust version
rustc --version

# Check QEMU
qemu-system-x86_64 --version

# Check OVMF location
ls /usr/share/ovmf/
```

---

## Accessing Your Project

### From WSL2

Your Windows drives are automatically mounted at `/mnt/`:

```bash
# Navigate to your project
cd /mnt/e/OS

# You can now build directly
cargo build --package cartridge-kernel --target x86_64-unknown-none
```

### From Windows

You can also access WSL2 filesystem from Windows Explorer:

```
\\wsl$\Ubuntu\home\<username>\
```

**Recommendation:** Keep your project on the Windows filesystem (`E:\OS`) and access it from WSL2 via `/mnt/e/OS` for best performance and easy file sharing.

---

## Building Components

### Build Kernel

```bash
cd /mnt/e/OS

cargo build --release \
  --package cartridge-kernel \
  --target x86_64-unknown-none
```

### Build Bootloader

```bash
cargo build --release \
  --package cartridge-bootloader \
  --target x86_64-unknown-uefi
```

### Build Tooling (Optional in WSL)

```bash
# Can also build Windows executables if needed
cargo build --release \
  --package cartridge-packager \
  --package cartridge-verifier
```

---

## Testing in QEMU

### Quick Test

```bash
# Run with UEFI firmware
qemu-system-x86_64 \
  -bios /usr/share/ovmf/OVMF.fd \
  -drive format=raw,file=boot.img \
  -m 512M \
  -serial stdio \
  -display none
```

### With Graphics

```bash
# Run with VGA output
qemu-system-x86_64 \
  -bios /usr/share/ovmf/OVMF.fd \
  -drive format=raw,file=boot.img \
  -m 512M \
  -serial stdio
```

---

## Performance Tips

### File Access Performance

- **Best:** Keep source on Windows filesystem, build from WSL2 (`/mnt/e/OS`)
- **Good:** Keep source in WSL2 filesystem (`~/projects/OS`)
- **Slow:** Editing files in WSL2 from Windows editors via `\\wsl$\`

### Memory Configuration

WSL2 uses dynamic memory allocation. To limit it, create `C:\Users\<YourUsername>\.wslconfig`:

```ini
[wsl2]
memory=4GB
processors=4
swap=2GB
```

### Disk Space

WSL2 disk image grows but doesn't shrink automatically. To reclaim space:

```powershell
# Shutdown WSL
wsl --shutdown

# Optimize disk (from PowerShell)
diskpart
# Then: select vdisk file="C:\Users\<user>\AppData\Local\Packages\...\ext4.vhdx"
# Then: compact vdisk
```

---

## IDE Integration

### VS Code with WSL

1. Install "Remote - WSL" extension
2. Open project in WSL:
   ```bash
   code /mnt/e/OS
   ```
3. VS Code will connect to WSL2 automatically

### Rust Analyzer

The Rust extension works seamlessly in WSL2:
- Full autocomplete
- Inline errors
- Cargo integration

---

## Troubleshooting

### WSL2 won't start
```powershell
# Reset WSL
wsl --shutdown
wsl
```

### Can't find Windows files
```bash
# Check mounted drives
ls /mnt/
# Should show c, d, e, etc.
```

### OVMF not found
```bash
# Find OVMF location
find /usr -name "OVMF*.fd" 2>/dev/null

# Common locations:
# /usr/share/ovmf/OVMF.fd
# /usr/share/OVMF/OVMF_CODE.fd
```

### Build errors with rust-src
```bash
# Reinstall rust-src
rustup component remove rust-src
rustup component add rust-src
```

---

## Alternative: Full Linux VM

If WSL2 doesn't meet your needs, you can use a full Linux VM:

**Pros:**
- Complete isolation
- Full control over kernel
- Can test actual hardware passthrough

**Cons:**
- More resource usage
- Slower file sharing
- More setup complexity

**Recommended VMs:**
- VirtualBox (free)
- VMware Workstation
- Hyper-V (Windows Pro/Enterprise)

---

## Next Steps

1. Follow `docs/BUILD.md` to build the kernel
2. Create your first bootable image
3. Test in QEMU
4. Start developing your first cartridge!

For more details, see:
- `docs/BUILD.md` - Complete build instructions
- `docs/DEVELOPMENT.md` - Development roadmap
- `README.md` - Project overview
