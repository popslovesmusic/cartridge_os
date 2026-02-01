# Development Roadmap

This document outlines the implementation roadmap for Cartridge OS.

## Phase 0: Scaffolding ✓ COMPLETE

- [x] Project structure and workspace setup
- [x] Common artifact format library
- [x] Kernel stub with subsystem placeholders
- [x] Bootloader stub with verification flow
- [x] Switcher stub with loading logic
- [x] Packager and verifier tooling
- [x] Build configuration for cross-compilation
- [x] Documentation (README, BUILD, WSL2_SETUP)

---

## Phase 1: Minimal Viable Kernel

### 1.1 Memory Management
- [ ] Implement physical memory allocator
- [ ] Page table setup and virtual memory
- [ ] Isolated memory regions for cartridges
- [ ] Shared memory window mapping (zero-copy IPC)

### 1.2 Syscall Interface
- [ ] Define syscall ABI (x86_64 calling convention)
- [ ] Implement syscall dispatcher in kernel
- [ ] Add basic syscalls:
  - `sys_log` - kernel logging
  - `sys_alloc` - memory allocation
  - `sys_exit` - terminate cartridge

### 1.3 Basic Scheduler
- [ ] Single-cartridge scheduler
- [ ] Context switching
- [ ] CPU core pinning

**Milestone:** Kernel can boot, allocate memory, and handle syscalls

---

## Phase 2: Bootloader and Chain of Trust

### 2.1 UEFI Bootloader
- [ ] Initialize UEFI boot services
- [ ] Read kernel artifact from ESP (EFI System Partition)
- [ ] Verify kernel hash
- [ ] Load kernel into memory at correct address
- [ ] Set up initial page tables
- [ ] Transfer control to kernel entry point

### 2.2 Signature Verification
- [ ] Integrate Ed25519 signature verification
- [ ] Generate and manage signing keys
- [ ] Sign kernel artifact during packaging
- [ ] Verify signature in bootloader

**Milestone:** Bootloader can verify and boot a signed kernel

---

## Phase 3: Switcher and Cartridge Loading

### 3.1 Switcher Implementation
- [ ] Discover cartridges from storage partition
- [ ] Parse cartridge manifests
- [ ] Verify cartridge artifacts
- [ ] Request kernel to allocate cartridge memory (syscall)
- [ ] Load cartridge into isolated region
- [ ] Transfer control to cartridge entry point

### 3.2 Example Cartridge: "Hello World"
- [ ] Create minimal bare-metal Rust application
- [ ] Print to serial/screen via syscalls
- [ ] Package as cartridge artifact
- [ ] Test full boot chain: bootloader → kernel → switcher → cartridge

**Milestone:** Complete boot chain with runnable cartridge

---

## Phase 4: Capability System

### 4.1 Capability Enforcement
- [ ] Capability table in kernel
- [ ] Grant capabilities at cartridge load time
- [ ] Check capabilities on syscalls/IPC
- [ ] Revoke capabilities on cartridge exit

### 4.2 Driver Framework
- [ ] Define driver interface (IPC protocol)
- [ ] Implement GPU driver stub (framebuffer access)
- [ ] Implement input driver stub (keyboard/mouse)
- [ ] Load drivers before cartridge based on capability requests

**Milestone:** Cartridge can access hardware via capability-gated drivers

---

## Phase 5: IPC and Drivers

### 5.1 IPC Channels
- [ ] Create channel syscall
- [ ] Send/receive message syscalls
- [ ] Channel cleanup on cartridge exit

### 5.2 Real Drivers
- [ ] Framebuffer driver (GPU capability)
- [ ] Audio driver (AUDIO capability)
- [ ] Input driver (INPUT capability)
- [ ] Storage driver (STORAGE capability)

### 5.3 Example Cartridge: Graphics Demo
- [ ] Request GPU capability
- [ ] Open IPC channel to framebuffer driver
- [ ] Draw pixels via shared memory
- [ ] Handle input events

**Milestone:** Cartridge with graphics and input working

---

## Phase 6: Storage and Persistence

### 6.1 Storage Partition
- [ ] Define storage layout (partitions A/B/C/D)
- [ ] Read-only artifact partitions
- [ ] Optional encrypted state vault (Partition D)

### 6.2 Persistent State
- [ ] Cartridge-scoped state storage
- [ ] Encrypt state vault with cartridge-specific key
- [ ] Prevent cross-cartridge state leakage

**Milestone:** Cartridge can save and load persistent data

---

## Phase 7: Testing and Hardening

### 7.1 Integration Tests
- [ ] Boot chain tests (verify → load → execute)
- [ ] Capability enforcement tests (deny-by-default)
- [ ] IPC throughput tests
- [ ] Memory isolation tests (prevent cross-cartridge access)

### 7.2 Fuzzing
- [ ] Fuzz artifact parser
- [ ] Fuzz syscall interface
- [ ] Fuzz IPC message handling

### 7.3 Performance
- [ ] Boot time optimization (< 5 seconds target)
- [ ] IPC latency benchmarks
- [ ] Zero-copy shared memory validation

**Milestone:** System is hardened and tested

---

## Phase 8: Tooling and UX

### 8.1 Cartridge SDK
- [ ] Rust crate for cartridge development
- [ ] Helper macros for syscalls
- [ ] Driver client libraries (GPU, audio, input)

### 8.2 Build Scripts
- [ ] `create-image.sh` - build bootable disk image
- [ ] `package-cartridge.sh` - wrap packager tool
- [ ] `run-qemu.sh` - simplified QEMU testing

### 8.3 Documentation
- [ ] Cartridge development guide
- [ ] Driver development guide
- [ ] Security audit documentation
- [ ] Performance tuning guide

**Milestone:** Developers can easily create cartridges

---

## Phase 9: Real Hardware

### 9.1 Hardware Bringup
- [ ] Test on real x86_64 hardware
- [ ] Driver support for common hardware (Intel/AMD GPU, Realtek audio, etc.)
- [ ] USB support for input devices

### 9.2 Installer
- [ ] Bootable USB image creator
- [ ] Partition setup tool
- [ ] Cartridge installation utility

**Milestone:** Cartridge OS runs on real hardware

---

## Phase 10: Advanced Features

### 10.1 Multiple Cartridges
- [ ] Cartridge switcher UI
- [ ] Fast cartridge switching (< 1 second)
- [ ] Shared state between cartridges (opt-in)

### 10.2 Network Stack (Optional)
- [ ] Userspace TCP/IP stack
- [ ] Network driver
- [ ] Capability-gated network access

### 10.3 Deterministic Execution
- [ ] CPU core isolation
- [ ] Predictable scheduling
- [ ] Real-time performance guarantees

**Milestone:** Production-ready appliance platform

---

## Long-Term Vision

- **Formal Verification**: Prove kernel correctness (like seL4)
- **Minimal TCB**: Reduce kernel to < 10k SLOC
- **Hardware Root of Trust**: TPM integration for key storage
- **Multi-Architecture**: ARM64, RISC-V support
- **Toolchain Integration**: Custom LLVM backend for cartridge compilation

---

## How to Contribute

Pick a task from Phase 1 or Phase 2 and:
1. Open an issue to claim the task
2. Implement and test in QEMU
3. Submit a pull request with tests

See **[README.md](../README.md)** for code style and contribution guidelines.
