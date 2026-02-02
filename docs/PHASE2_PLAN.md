# Phase 2: Bootable System - Execution Plan

**Status:** Ready to Begin
**Based on:** Senior Technical Architect evaluation
**Token Budget:** 82k remaining (plenty)

---

## Architect's Priority Guidance

From `evaluation of the current state and the path forward.txt`:

### ✅ Phase 1 Approved
- **4.3 KB kernel** - Exceptionally lean
- **O(1) allocator** - Deterministic for success metrics
- **Zero-copy IPC** - Bare metal speed for GPU
- **8 syscalls** - Clean, narrow interface

### 🎯 Phase 2 Priorities (in order)

1. **Serial Logging** - "Your only window into the kernel's brain"
2. **UEFI Bootloader** - Verify kernel hash before handoff
3. **"Hello World" Cartridge** - Tanka-headered proof-of-concept
4. **Chain of Trust** - Firmware → Bootloader → Kernel → Switcher

---

## Implementation Order

### Task 1: Serial Port Driver (PRIORITY 1)
**Rationale:** Without screen driver, serial is our only debugging channel

**Implementation:**
```rust
// kernel/src/serial.rs
- COM1 port initialization (0x3F8)
- Write byte function
- Write string function
- Integration with kernel_log()
```

**Success Criteria:**
- Kernel can output to serial port
- Visible in QEMU with `-serial stdio`

**Estimated:** ~150 SLOC, ~5k tokens

---

### Task 2: UEFI Bootloader (PRIORITY 2)
**Rationale:** "Doesn't just load but **verifies the kernel's hash**"

**Implementation:**
```rust
// bootloader/src/main.rs
1. Initialize UEFI boot services
2. Load kernel artifact from ESP
3. Parse artifact header
4. Verify SHA-256 hash
5. Load at 1MB physical address
6. Set up initial page tables
7. Transfer control to kernel _start
```

**Success Criteria:**
- Boots in QEMU
- Rejects tampered kernels
- Logs to serial before handoff

**Estimated:** ~300 SLOC, ~15k tokens

---

### Task 3: Bootable Disk Image Script
**Rationale:** Need actual bootable media for QEMU

**Implementation:**
```bash
# scripts/create-image.sh
1. Create GPT disk image (128MB)
2. Create ESP partition (64MB, FAT32)
3. Copy bootloader as /EFI/BOOT/BOOTX64.EFI
4. Copy kernel as /kernel.bin
5. Create artifact partition (read-only)
```

**Success Criteria:**
- `boot.img` created successfully
- Recognized by QEMU OVMF firmware
- Bootloader launches

**Estimated:** ~100 lines script, ~5k tokens

---

### Task 4: "Hello World" Cartridge (PRIORITY 3)
**Architect Quote:** *"A Tanka-headered module that prints to the serial port"*

**Implementation:**
```rust
// cartridges/hello/src/main.rs
1. Bare-metal Rust binary
2. Uses sys_log syscall
3. Prints Tanka to serial
4. Exits cleanly
```

**Tanka Example:**
```
Kernel protects all    (5)
Cartridge runs when verified (7)
Trust enforced first   (5)
Serial port speaks the truth (7)
Hello from userspace here (7)
```

**Success Criteria:**
- Compiles to sealed artifact
- Tanka validation passes
- Switcher loads and executes
- Output visible on serial

**Estimated:** ~100 SLOC, ~8k tokens

---

### Task 5: Switcher Implementation
**Rationale:** Bridge between kernel and cartridges

**Implementation:**
```rust
// switcher/src/main.rs
1. Called by kernel after init
2. Discover cartridges (hardcoded path for now)
3. Verify cartridge artifact
4. Syscall to allocate memory
5. Load cartridge into memory
6. Syscall to transfer control
```

**Success Criteria:**
- Finds hello.cart
- Verifies integrity
- Executes cartridge
- Cartridge output appears

**Estimated:** ~200 SLOC, ~12k tokens

---

## Success Metrics (from spec)

| Metric | Target | How We'll Test |
|--------|--------|----------------|
| **Verified boot** | 100% success | Tamper kernel → boot fails |
| **Tamper detection** | Any byte change | Modify kernel → rejected |
| **Boot time** | <5 seconds | Time from QEMU start to cartridge output |
| **Recovery** | Auto restore | Boot fails → kernel doesn't load |

---

## Testing Strategy

### Level 1: Serial Output
```bash
# Run in QEMU with serial
qemu-system-x86_64 -bios OVMF.fd -drive file=boot.img -serial stdio

# Expected output:
[BOOTLOADER] Cartridge OS Bootloader
[BOOTLOADER] Loading kernel...
[BOOTLOADER] Verifying kernel hash...
[BOOTLOADER] Kernel verified, transferring control
[KERNEL] Cartridge OS Kernel initialized
[KERNEL] Memory subsystem initialized
[KERNEL] IPC subsystem initialized
[KERNEL] Syscall subsystem initialized
[SWITCHER] Switcher started
[SWITCHER] Found cartridge: hello.cart
[SWITCHER] Verifying cartridge...
[SWITCHER] Cartridge verified, executing
[CARTRIDGE] Hello from userspace here
[CARTRIDGE] Exiting cleanly
```

### Level 2: Tamper Detection
```bash
# Corrupt kernel
dd if=/dev/urandom of=kernel.bin bs=1 count=10 seek=100 conv=notrunc

# Rebuild image
./scripts/create-image.sh

# Boot should fail:
[BOOTLOADER] Kernel verification failed: hash mismatch
[BOOTLOADER] HALT
```

### Level 3: Boot Time
```bash
# Time from QEMU start to cartridge output
time qemu-system-x86_64 ... | grep "Hello from userspace"

# Target: < 5 seconds
```

---

## Architectural Considerations

### Chain of Trust
```
UEFI Firmware
  ↓ (verify bootloader signature - future)
Bootloader
  ↓ (verify kernel SHA-256)
Kernel
  ↓ (verify switcher - future)
Switcher
  ↓ (verify cartridge SHA-256)
Cartridge
  ↓ (execute)
```

**Phase 2 Scope:** Bootloader → Kernel verification (bold line)

### Memory Layout
```
0x00000000 - 0x00100000  : Reserved (firmware, bootloader)
0x00100000 - 0x00200000  : Kernel (loaded here by bootloader)
0x00200000 - 0x00300000  : Switcher (loaded by kernel)
0x00300000 - 0x10000000  : Cartridge regions (allocated on demand)
```

### Serial Port Details
```
COM1: 0x3F8 (standard x86 serial port)
Baud: 115200
Data: 8 bits
Parity: None
Stop: 1 bit
```

---

## Implementation Sequence

### Week 1 (or Session 1)
1. ✅ Serial port driver (~5k tokens)
2. ✅ Test serial output in kernel init
3. ✅ UEFI bootloader skeleton (~8k tokens)
4. ✅ Test bootloader loads kernel

### Week 2 (or Session 2)
5. ✅ Add kernel hash verification (~5k tokens)
6. ✅ Test tamper detection works
7. ✅ Create bootable image script (~5k tokens)
8. ✅ Boot in QEMU successfully

### Week 3 (or Session 3)
9. ✅ Implement switcher (~12k tokens)
10. ✅ Create hello world cartridge (~8k tokens)
11. ✅ Test full boot chain
12. ✅ Measure boot time (<5s?)

**Total Estimated:** ~48k tokens (well within 82k budget)

---

## Documentation Requirements

Per "documentation as shining star #2":

### New Documents
1. `docs/PHASE2_SERIAL.md` - Serial driver deep dive
2. `docs/PHASE2_BOOTLOADER.md` - UEFI bootloader guide
3. `docs/PHASE2_TESTING.md` - QEMU testing procedures
4. `docs/CARTRIDGE_SDK.md` - How to create cartridges (basic)

### Updates
1. `README.md` - Phase 2 status
2. `docs/BUILD.md` - Add QEMU instructions
3. `docs/DEVELOPMENT.md` - Mark Phase 2 complete

**Estimated:** ~6,000 words additional documentation

---

## Risk Mitigation

### Risk 1: UEFI Complexity
**Mitigation:** Use `uefi-rs` crate for Rust UEFI support
**Fallback:** Minimal C bootloader if Rust too complex

### Risk 2: Page Table Setup
**Mitigation:** Identity map first 4GB for simplicity
**Fallback:** Kernel runs in same address space as bootloader

### Risk 3: Boot Time >5s
**Mitigation:** Optimize verification (SHA-256 is fast)
**Fallback:** Accept slightly over 5s, optimize in Phase 3

### Risk 4: QEMU vs Real Hardware
**Mitigation:** Test in QEMU first, real hardware in Phase 9
**Fallback:** Document QEMU-specific workarounds

---

## Architect's Specific Requirements

### 1. Hash Verification (not just loading)
```rust
// bootloader verification
let kernel_data = load_kernel_from_esp()?;
let artifact = Artifact::from_bytes(&kernel_data)?;

// This already verifies hash!
if artifact.header.artifact_type != ArtifactType::Kernel {
    panic!("Invalid artifact type");
}

// Hash was checked in Artifact::from_bytes()
// If we reach here, kernel is verified ✓
```

### 2. Serial as "Only Window"
```rust
// All kernel_log() calls route to serial
pub fn kernel_log(msg: &str) {
    serial::write_string(msg);
    serial::write_string("\n");
}

// Panic also goes to serial
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel_log("PANIC!");
    // ... details to serial
}
```

### 3. "Silly" Hello World
```json
// hello_tanka.json
{
  "lines": [
    "Kernel protects all",
    "Cartridge runs when verified",
    "Trust enforced first",
    "Serial port speaks the truth",
    "Hello from userspace here"
  ]
}
```

---

## Phase 2 Success Criteria

- [ ] Serial port outputs kernel messages
- [ ] Bootloader verifies kernel hash
- [ ] Bootloader rejects corrupted kernel
- [ ] Kernel boots in QEMU
- [ ] Switcher loads cartridge
- [ ] Hello world cartridge executes
- [ ] Output visible on serial
- [ ] Boot time measured (<5s target)
- [ ] Documentation complete

---

## Ready to Begin?

**Recommendation:** Start with Task 1 (Serial Driver)

This is:
- Small scope (~150 SLOC)
- Immediate value (debugging)
- Architect's priority #1
- Low risk

**Next Command:**
```
Implement serial port driver for kernel debugging
```

Shall we proceed with the serial driver?
