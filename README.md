# Cartridge OS

> **A microkernel-based appliance operating system built for deterministic, single-cartridge execution with verification-first security.**

📚 **[Complete Documentation Index](README_DOCS.md)** | 🚀 **[Quick Start](QUICKSTART.md)** | 🏗️ **[Build Guide](docs/BUILD.md)**

---

## Overview

Cartridge OS is not a general-purpose operating system. It is a **purpose-built appliance platform** that:
- Boots directly into a single verified application ("cartridge")
- Enforces capability-based security at every layer
- Provides deterministic execution for critical workloads
- Maintains an immutable chain of trust from firmware to application

### Core Principles

1. **Verification always precedes execution**
2. **Immutability by default** - no in-place mutation of sealed artifacts
3. **Capability-based security** - explicit hardware access grants only
4. **Minimal trusted computing base** - kernel <1MB
5. **Single cartridge at a time** - no multi-tasking complexity

---

## Architecture

```
┌─────────────────────────────────────────┐
│         Application Cartridge           │
│    (verified, isolated, capability-gated)│
└────────────────┬────────────────────────┘
                 │
┌────────────────┴────────────────────────┐
│           Kernel (Microcore)            │
│   Memory | Capabilities | IPC | Sched   │
└────────────────┬────────────────────────┘
                 │
┌────────────────┴────────────────────────┐
│              Switcher                   │
│   (loads & verifies cartridges)         │
└────────────────┬────────────────────────┘
                 │
┌────────────────┴────────────────────────┐
│            Bootloader (UEFI)            │
│      (verifies kernel signature)        │
└─────────────────────────────────────────┘
```

### Components

- **Bootloader**: UEFI application that verifies kernel integrity
- **Kernel**: Minimal microkernel providing memory isolation, IPC, and scheduling
- **Switcher**: Userspace component that loads and verifies cartridges
- **Drivers**: Userspace modules mediating hardware access
- **Cartridges**: Application packages with declared capabilities

---

## Getting Started

### Prerequisites

- **Windows** (for initial development)
- **Rust nightly toolchain**
- **WSL2** (for building bootable images)
- **QEMU** (for testing, via WSL2)

### Quick Start

#### 1. Clone and Setup

```powershell
cd E:\OS

# Install Rust nightly
rustup toolchain install nightly
rustup component add rust-src rustfmt clippy
rustup target add x86_64-unknown-uefi
```

#### 2. Build Tooling (Windows)

```powershell
# Build packager and verifier
cargo build --release --package cartridge-packager --package cartridge-verifier

# Test tooling
.\target\release\cartridge-packager.exe --help
```

#### 3. Build Kernel (WSL2)

See **[docs/WSL2_SETUP.md](docs/WSL2_SETUP.md)** for WSL2 installation, then:

```bash
cd /mnt/e/OS

# Build kernel
cargo build --release --package cartridge-kernel --target x86_64-unknown-none

# Build bootloader
cargo build --release --package cartridge-bootloader --target x86_64-unknown-uefi
```

#### 4. Test in QEMU

```bash
# Create bootable image (script coming soon)
./scripts/create-image.sh

# Run in QEMU
qemu-system-x86_64 \
  -bios /usr/share/ovmf/OVMF.fd \
  -drive format=raw,file=boot.img \
  -m 512M -serial stdio
```

---

## Documentation

- **[specification-sheet-detailed.md](specification-sheet-detailed.md)** - System specification and invariants
- **[technical-blueprint-detailed.md](technical-blueprint-detailed.md)** - Architecture and implementation details
- **[docs/BUILD.md](docs/BUILD.md)** - Complete build instructions
- **[docs/WSL2_SETUP.md](docs/WSL2_SETUP.md)** - WSL2 setup for Windows users
- **[docs/DEVELOPMENT.md](docs/DEVELOPMENT.md)** - Development roadmap (coming soon)

---

## Project Structure

```
E:\OS\
├── kernel/              # Microkernel (<1MB bare-metal binary)
├── bootloader/          # UEFI bootloader (verifies kernel)
├── switcher/            # Cartridge loader (userspace)
├── common/              # Shared artifact format library
├── tooling/
│   ├── packager/        # CLI tool to create sealed artifacts
│   └── verifier/        # CLI tool to verify artifacts
├── drivers/             # Driver modules (future)
├── cartridges/          # Example cartridges (future)
├── docs/                # Documentation
└── tests/               # Integration tests (future)
```

---

## Artifact Format

All executable units (kernel, drivers, cartridges) share a common artifact format:

```
┌──────────────────────────────────┐
│     Header (128 bytes)           │
│  - Magic: 0xCAFEBABE             │
│  - Version                       │
│  - Entry point offset            │
│  - Capability flags              │
│  - SHA-256 payload hash          │
└──────────────────────────────────┘
│     Payload (executable + data)  │
└──────────────────────────────────┘
```

### Creating Artifacts

```bash
# Package a cartridge
cartridge-packager cartridge \
  --input my-app.elf \
  --output my-app.cart \
  --capabilities GPU,AUDIO,INPUT

# Verify artifact
cartridge-verifier my-app.cart --verbose
```

---

## Development Status

**Current Phase: Phase 1 Complete ✓** ([Full Report](docs/PHASE1_COMPLETE.md))

### ✅ Completed (Phase 1)
- [x] Deterministic memory allocator (O(1), page-aligned)
- [x] Zero-copy shared memory IPC (AVX2-aligned)
- [x] Syscall interface (8 core syscalls)
- [x] Tanka syllable validation (build-time enforcement)
- [x] Kernel size verification (4.3KB = 0.41% of 1MB limit)
- [x] Comprehensive documentation (12,000+ words)
- [x] Artifact packager (working, tested)
- [x] Artifact verifier (working, tested)

**Architect Approval:** ✅ APPROVED for Phase 2

### ⏳ Next (Phase 2 - Bootable System)
- [ ] UEFI bootloader implementation
- [ ] Disk image creation script
- [ ] QEMU boot testing
- [ ] Switcher implementation
- [ ] First "Hello World" cartridge

**Token Budget:** 113k / 200k used (56%) - plenty remaining

---

## Design Philosophy

### What Cartridge OS IS

- A **specialized appliance platform** for single-purpose execution
- A **verification-first system** where trust is explicit
- A **capability-based security model** with no ambient authority
- A **deterministic runtime** for critical workloads

### What Cartridge OS IS NOT

- Not a desktop OS
- Not multi-user
- Not a container runtime
- Not POSIX-compatible
- Not a general-purpose server

---

## Contributing

This project is currently in early development. Contributions welcome!

### Development Workflow

1. Fork the repository
2. Create a feature branch
3. Make changes (follow Rust conventions)
4. Test on WSL2/QEMU
5. Submit pull request

### Code Style

- Follow Rust 2021 edition idioms
- Use `rustfmt` for formatting
- Run `clippy` for linting
- Document public APIs

---

## License

MIT OR Apache-2.0 (dual-licensed)

---

## Contact

For questions or discussion, open an issue on the repository.

---

## Acknowledgments

Inspired by:
- **Redox OS** - Microkernel design in Rust
- **seL4** - Formal verification and capability systems
- **Genode** - Component-based OS architecture
- **UEFI** - Modern firmware interface
