# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**Cartridge OS** is a microkernel-based **appliance operating system**, not a general-purpose OS. It is designed to boot directly into a single verified application ("cartridge") with deterministic execution and immutable chain of trust.

### Core Philosophy
- **Verification always precedes execution** - SHA-256 hash checking at every layer
- **Immutability by default** - No in-place mutation of sealed artifacts
- **Capability-based security** - Explicit hardware access grants only
- **Minimal TCB** - Kernel target: <128KB (currently 8.8KB), fitting in L2 cache
- **Single cartridge at a time** - No multi-tasking complexity
- **L2/SIMD optimization** - Designed for cache-line aligned (64-byte) SIMD throughput, not just code size

### What This IS NOT
- Not a desktop OS, not multi-user, not POSIX-compatible
- Not a general-purpose server or container runtime
- This is a **specialized appliance platform** for single-purpose execution

## Critical Known Issues

**WARNING**: The system currently has **9 fatal boot issues** preventing USB boot. See `docs/CODE_REVIEW_CRITICAL_ISSUES.md` for full details.

Most critical issue: Entry point offset is hardcoded to 0 in the packager, causing bootloader to jump to ELF header instead of code, resulting in instant triple-fault reboot.

**Priority fixes needed** (in order):
1. Fix entry point calculation in `tooling/packager/src/main.rs`
2. Setup IDT in kernel for exception handling
3. Zero BSS section in kernel startup
4. Allocate proper kernel stack in bootloader
5. Fix memory allocation type in bootloader

**Testing strategy**: Use QEMU with OVMF first, NOT USB drives. See testing section below.

## Build Commands

### Prerequisites
- **Windows**: Rust nightly, Visual Studio Build Tools
- **WSL2/Linux**: Rust nightly, QEMU, OVMF firmware, build-essential

### Building Components

```bash
# Build host tooling (Windows or Linux)
cargo build --release --package cartridge-packager --package cartridge-verifier

# Build kernel (bare-metal x86_64)
cargo build --release --package cartridge-kernel --target x86_64-unknown-none

# Build bootloader (UEFI)
cargo build --release --package cartridge-bootloader --target x86_64-unknown-uefi

# Build switcher (bare-metal userspace)
cargo build --release --package cartridge-switcher --target x86_64-unknown-none

# Cargo aliases (from .cargo/config.toml)
cargo build-kernel      # Builds kernel only
cargo build-bootloader  # Builds bootloader only
cargo build-tools       # Builds packager + verifier for Windows
```

### Package Kernel as Artifact

```bash
# After building kernel, package it with hash verification
cargo run --package cartridge-packager -- kernel \
    --input target/x86_64-unknown-none/release/cartridge-kernel \
    --output usb-files/kernel.bin

# Package a cartridge with capabilities
cargo run --package cartridge-packager -- cartridge \
    --input path/to/app.elf \
    --output app.cart \
    --capabilities GPU,AUDIO,INPUT \
    --tanka tanka.json

# Verify artifact integrity
cargo run --package cartridge-verifier -- usb-files/kernel.bin --verbose
```

### Testing in QEMU

```bash
# (WSL2/Linux only) Create bootable disk image
./scripts/create-disk-image.sh

# Boot in QEMU with serial output
qemu-system-x86_64 \
    -drive format=raw,file=cartridge-os.img \
    -bios /usr/share/ovmf/OVMF.fd \
    -serial stdio \
    -m 256M \
    -no-reboot \
    -d int,cpu_reset  # Shows crash details
```

## Architecture

### Boot Chain
```
Firmware (UEFI)
    ↓
Bootloader (UEFI app)
    - Loads /kernel.bin from ESP
    - Verifies SHA-256 hash
    - HALTS on mismatch (security critical)
    - Allocates kernel at 0x100000 (1MB)
    - Exits boot services
    - Jumps to kernel entry point
    ↓
Kernel (microkernel <1MB)
    - Initializes serial (COM1) FIRST
    - Memory allocator (bump, deterministic)
    - IPC (zero-copy shared memory)
    - Syscall interface (8 syscalls)
    - Capability enforcement
    ↓
Switcher (userspace)
    - Loads cartridges
    - Verifies cartridge artifacts
    - Manages cartridge lifecycle
    ↓
Cartridge (application)
    - Single-purpose app
    - Capability-gated hardware access
```

### Component Responsibilities

**Bootloader** (`bootloader/`):
- UEFI application that loads and verifies kernel
- Uses `uefi` crate v0.30 (NOT deprecated `uefi-services`)
- Artifact format: loads kernel wrapped in 128-byte header with SHA-256

**Kernel** (`kernel/`):
- Minimal microkernel (~8.8KB currently, target <128KB)
- **Critical**: Serial driver (`serial.rs`) initialized FIRST - only debug output mechanism
- Memory allocator uses atomic bump pointer (L2-optimized for SIMD)
- No paging implemented yet (fatal security issue)
- No IDT setup yet (fatal crash issue)

**Switcher** (`switcher/`):
- Userspace component that loads cartridges
- Never actually loaded by kernel yet (stub implementation)

**Common** (`common/`):
- Shared artifact format library
- Header: 128 bytes (magic 0xCAFEBABE, version, entry offset, capabilities, SHA-256)
- All executables (kernel, drivers, cartridges) use this format

**Tooling** (`tooling/`):
- `packager`: Creates sealed artifacts with hash verification
- `verifier`: Validates artifact integrity
- **Build for Windows host**: `--target x86_64-pc-windows-msvc`

### Artifact Format

Every executable (kernel, driver, cartridge) is wrapped in this format:

```
┌──────────────────────────────────┐
│     Header (128 bytes)           │
│  - Magic: 0xCAFEBABE             │
│  - Version: u16                  │
│  - Artifact type: u8 (0=Kernel)  │
│  - Entry offset: u64             │  ⚠️ Currently hardcoded to 0 (BUG)
│  - Capability flags: u64         │
│  - Payload size: u64             │
│  - SHA-256 hash: [u8; 32]        │
│  - Header checksum: u32          │
└──────────────────────────────────┘
│     Payload (ELF binary)         │
└──────────────────────────────────┘
```

**Capabilities** (bitflags):
- GPU, AUDIO, INPUT, NETWORK, STORAGE, USB
- IPC_BASIC, IPC_SHARED_MEMORY
- REBOOT

**Tanka Metadata** (5-7-5-7-7 syllable structure):
- Optional poetry describing cartridge purpose
- Non-authoritative (human-facing only)
- Validated at build time by packager

## File Organization

### Linker Scripts
Each bare-metal component has its own linker script:
- `kernel/linker.ld` - Loads at 1MB (0x100000)
- `switcher/linker.ld` - Loads at 2MB (0x200000)
- `cartridges/*/linker.ld` - Loads at 4MB+

**Critical**: Each component overrides global rustflags in its own `Cargo.toml` to specify linker script path.

### Memory Layout
- **0x000000**: Reserved (real mode IVT, BIOS data)
- **0x100000** (1MB): Kernel load address
- **0x200000** (2MB): Switcher load address (if loaded)
- **4MB+**: Cartridge memory regions

### Build Targets
- `x86_64-unknown-none`: Bare-metal (kernel, switcher, cartridges)
- `x86_64-unknown-uefi`: UEFI bootloader
- `x86_64-pc-windows-msvc`: Host tooling (packager, verifier)

## Development Workflow

### Phase Status
- **Phase 1**: ✅ Complete - Scaffold and tooling
- **Phase 2**: ⚠️ Claimed complete but **will not boot** (9 fatal bugs)
- **Phase 3**: Not started - Hardware drivers

### Making Changes to Kernel

1. **Always preserve serial initialization order**:
   ```rust
   #[no_mangle]
   pub extern "C" fn _start() -> ! {
       serial::init();  // MUST BE FIRST
       serial_println!("[KERNEL] Starting...");
       // ... rest of initialization
   }
   ```

2. **All kernel output goes to serial (COM1)** - no screen driver yet
3. **Maintain no_std** - kernel has no standard library
4. **Watch binary size**: Target <128KB (currently 8.8KB)

### Making Changes to Bootloader

1. **Use `uefi` crate v0.30**, NOT `uefi-services` (deprecated)
2. **Initialize with**: `uefi::helpers::init(&mut system_table)`
3. **Runtime strings**: Use `CString16::try_from(msg)`, NOT `cstr16!(msg)` macro
4. **File operations**: Convert `FileHandle` → `RegularFile` before reading
5. **Borrow checker**: Use block scoping to limit `system_table` borrow lifetimes

### Artifact Packaging Workflow

```bash
# 1. Build the binary
cargo build --release --package cartridge-kernel --target x86_64-unknown-none

# 2. Package with hash (creates artifact header)
cargo run --package cartridge-packager -- kernel \
    --input target/x86_64-unknown-none/release/cartridge-kernel \
    --output usb-files/kernel.bin

# 3. Verify integrity
cargo run --package cartridge-verifier -- usb-files/kernel.bin

# 4. Copy bootloader to USB structure
cp target/x86_64-unknown-uefi/release/cartridge-bootloader.efi \
   usb-files/EFI/BOOT/BOOTX64.EFI

# 5. Test in QEMU first (WSL2/Linux)
./scripts/create-disk-image.sh
qemu-system-x86_64 -drive format=raw,file=cartridge-os.img \
    -bios /usr/share/ovmf/OVMF.fd -serial stdio -m 256M
```

## Performance Considerations

### L2 Cache Optimization Strategy

This OS is optimized for **L2 cache + SIMD lanes**, not just code size:

- **Target**: Kernel <128KB (fits in 256KB L2 cache on Xeon)
- **Focus**: Cache-line aligned data structures (64-byte boundaries)
- **Goal**: Maximum AVX2/SIMD throughput for research workloads
- **Reality**: Hot path (frequent code) stays in L1, data working set in L2

### Memory Alignment
- **IPC shared memory**: 32-byte aligned (AVX2)
- **Page tables**: 4KB aligned
- **Cache lines**: 64-byte aligned on Xeon

### Zero-Copy IPC
- Shared memory windows for GPU/audio (high throughput)
- AVX2-aligned buffers for SIMD operations
- Non-temporal stores (`vmovntdq`) for large transfers

## Common Issues

### Build Errors

**"linker script not found"**:
- Check that component's `Cargo.toml` specifies correct linker script path
- Each bare-metal component needs its own linker script

**"uefi-services crate not found"**:
- Use `uefi = "0.30"` with `uefi::helpers::init()` instead
- Do NOT use deprecated `uefi-services`

**"cannot find macro cstr16"**:
- Add `use uefi::cstr16;` for compile-time strings
- Use `CString16::try_from()` for runtime string conversion

### Runtime Issues

**Serial output not appearing**:
- Ensure QEMU has `-serial stdio` flag
- Check that `serial::init()` is called FIRST in kernel
- Real hardware needs COM1 port or USB-to-serial adapter (115200 baud)

**QEMU triple-fault / instant reboot**:
- Entry point offset is wrong (known bug in packager)
- No IDT setup (any exception causes triple-fault)
- Check QEMU log with `-d int,cpu_reset`

**Hash verification failure in bootloader**:
- Rebuild kernel.bin after ANY kernel changes
- Don't manually edit kernel.bin
- Verify packager ran successfully

## Design Constraints

### Immutability
- Artifacts are sealed and immutable once packaged
- No in-place updates or patches
- To change: rebuild, repackage, reboot

### Size Limits
- **Kernel**: <128KB target (L2 cache), <1MB hard limit
- **Bootloader**: ~2KB (currently excellent)
- **Entire system**: Minimal TCB for auditability

### No Standard Library
- All bare-metal components are `#![no_std]`
- Use `alloc` for heap allocations (kernel provides allocator)
- No file I/O, no threading (single-cartridge model)

### Security Model
- Capability-based (explicit hardware access grants)
- Deny-by-default for all resources
- Hash verification at every boundary
- Hard halt on verification failure (no fallback)

## Important Documentation

- `docs/CODE_REVIEW_CRITICAL_ISSUES.md` - **READ FIRST** - Lists 9 fatal boot bugs
- `docs/specialized appliance.txt` - Architecture philosophy and L2/SIMD strategy
- `specification-sheet-detailed.md` - System invariants and trust model
- `technical-blueprint-detailed.md` - Implementation details
- `docs/PHASE2_COMPLETE.md` - Current status (claimed, but has fatal bugs)
- `READY_TO_BOOT.md` - Boot instructions (OUTDATED - will not boot)

## Testing Strategy

### QEMU First, USB Later
1. Fix critical bugs first (see CODE_REVIEW_CRITICAL_ISSUES.md)
2. Test in QEMU with `-d int,cpu_reset` to see crashes
3. Verify boot messages appear on serial
4. Only after QEMU success, try USB drives

### Expected Boot Sequence (When Fixed)
```
[BOOTLOADER] Cartridge OS Bootloader v0.1.0
[BOOTLOADER] Loading kernel from /kernel.bin
[BOOTLOADER] ✓ Kernel hash verified
[BOOTLOADER] Transferring control to kernel...

[KERNEL] Cartridge OS Kernel starting...
[KERNEL] IDT initialized
[KERNEL] Memory subsystem initialized
[KERNEL] All subsystems initialized
```

### Current Reality (Broken)
```
[BOOTLOADER] Transferring control to kernel...
(Instant reboot - jumps to ELF header, not code)
```
