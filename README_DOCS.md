# Cartridge OS Documentation Index

**Documentation Philosophy:** Second only to actual function - comprehensive, clear, and always up-to-date.

---

## 📚 Documentation Hierarchy

### 1. **Getting Started** (New Users)
- [README.md](README.md) - Project overview and quick start
- [QUICKSTART.md](QUICKSTART.md) - 10-minute setup guide
- [docs/BUILD.md](docs/BUILD.md) - Complete build instructions
- [docs/WSL2_SETUP.md](docs/WSL2_SETUP.md) - WSL2 setup for Windows users

### 2. **Specifications** (Design Authority)
- [specification-sheet-detailed.md](specification-sheet-detailed.md) - System specification and invariants
- [technical-blueprint-detailed.md](technical-blueprint-detailed.md) - Architecture and implementation details
- [docs/Senior Technical Architect.txt](docs/Senior%20Technical%20Architect.txt) - Architectural approval and guidance

### 3. **Implementation Guides** (Developers)
- [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) - 10-phase implementation roadmap
- [docs/PHASE1_COMPLETE.md](docs/PHASE1_COMPLETE.md) - Phase 1 completion report
- [docs/PHASE1_MEMORY.md](docs/PHASE1_MEMORY.md) - Memory allocator deep dive

### 4. **Reference** (Daily Use)
- [SCAFFOLD_COMPLETE.md](SCAFFOLD_COMPLETE.md) - Initial scaffolding status
- Build scripts documentation (below)
- API reference (coming in Phase 2)

---

## 🎯 Documentation by Task

### "I want to build Cartridge OS"
→ Start with [QUICKSTART.md](QUICKSTART.md)
→ Then [docs/BUILD.md](docs/BUILD.md)

### "I want to understand the architecture"
→ Read [specification-sheet-detailed.md](specification-sheet-detailed.md)
→ Then [technical-blueprint-detailed.md](technical-blueprint-detailed.md)
→ Review [docs/Senior Technical Architect.txt](docs/Senior%20Technical%20Architect.txt)

### "I want to contribute code"
→ Check [docs/DEVELOPMENT.md](docs/DEVELOPMENT.md) for roadmap
→ Review [docs/PHASE1_COMPLETE.md](docs/PHASE1_COMPLETE.md) for current status
→ Read the relevant specification sections

### "I want to create a cartridge"
→ (Coming in Phase 8 - Cartridge SDK documentation)

### "I want to understand the kernel internals"
→ Start with [docs/PHASE1_MEMORY.md](docs/PHASE1_MEMORY.md)
→ Then [docs/PHASE1_COMPLETE.md](docs/PHASE1_COMPLETE.md)
→ Review inline code documentation in `kernel/src/`

---

## 📖 Documentation Standards

### All Documentation Must Include:
1. **Purpose** - Why this document exists
2. **Audience** - Who should read this
3. **Prerequisites** - What you need to know first
4. **Examples** - Working code/commands
5. **Next Steps** - Where to go from here

### Code Documentation Standards:
- Every module has a header comment explaining its purpose
- Public functions documented with examples
- Complex algorithms explained inline
- TODO comments reference issue numbers (future)

---

## 🔍 Quick Reference

### Build Commands
```powershell
# Check kernel (Windows)
cargo check --package cartridge-kernel

# Build kernel (WSL2)
cargo build --release --package cartridge-kernel

# Verify kernel size
powershell -ExecutionPolicy Bypass -File scripts/check-kernel-size.ps1

# Build tools
cargo build --release --package cartridge-packager --target x86_64-pc-windows-msvc
cargo build --release --package cartridge-verifier --target x86_64-pc-windows-msvc
```

### Package a Cartridge
```bash
cartridge-packager cartridge \
  --input app.bin \
  --output app.cart \
  --capabilities GPU,AUDIO \
  --tanka app_tanka.json
```

### Verify an Artifact
```bash
cartridge-verifier app.cart --verbose
```

---

## 📝 Documentation Coverage

| Component | Spec | Implementation | Tutorial | API Ref |
|-----------|------|----------------|----------|---------|
| **Kernel** | ✓ | ✓ | Partial | ⏳ Phase 2 |
| **Memory** | ✓ | ✓ | ✓ | ⏳ Phase 2 |
| **IPC** | ✓ | ✓ | ⏳ Phase 2 | ⏳ Phase 2 |
| **Syscalls** | ✓ | ✓ | ⏳ Phase 2 | ⏳ Phase 2 |
| **Bootloader** | ✓ | ⏳ Phase 2 | ⏳ Phase 2 | ⏳ Phase 2 |
| **Tooling** | ✓ | ✓ | ✓ | ⏳ Phase 3 |
| **Tanka** | ✓ | ✓ | ✓ | ✓ |

**Legend:**
- ✓ = Complete
- Partial = Exists but needs expansion
- ⏳ = Planned (with phase number)

---

## 🚀 Future Documentation (Planned)

### Phase 2
- [ ] Bootloader Implementation Guide
- [ ] QEMU Testing Tutorial
- [ ] Boot Chain Debugging Guide
- [ ] Kernel API Reference (generated)

### Phase 3-5
- [ ] Driver Development Guide
- [ ] IPC Performance Tuning
- [ ] Capability System Reference
- [ ] Security Audit Documentation

### Phase 6-8
- [ ] Cartridge SDK Tutorial
- [ ] Tanka Writing Guide
- [ ] Performance Benchmarking Guide
- [ ] Troubleshooting Common Issues

### Phase 9-10
- [ ] Hardware Compatibility List
- [ ] Production Deployment Guide
- [ ] Network Stack Documentation
- [ ] Real-Time Performance Tuning

---

## 💡 Documentation Philosophy

> "Documentation is second only to actual function"

This means:
1. **Every feature gets documented** before it's "done"
2. **Documentation is tested** (commands must work)
3. **Examples are real** (not pseudocode)
4. **Kept up-to-date** (stale docs = broken docs)
5. **Accessible** (multiple skill levels)

---

## 🔗 External Resources

- [Rust no_std Book](https://docs.rust-embedded.org/book/)
- [OSDev Wiki](https://wiki.osdev.org/)
- [x86_64 Reference](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
- [UEFI Specification](https://uefi.org/specifications)

---

## 📧 Contributing to Documentation

1. Check this index for where the doc should go
2. Follow the documentation standards above
3. Test all examples/commands
4. Submit PR with documentation + code changes together

**Documentation is not optional - it's part of the feature.**

---

Last Updated: 2026-02-01 (Phase 1 Complete)
