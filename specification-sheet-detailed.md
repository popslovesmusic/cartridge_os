# Specification Sheet (Detailed)

## 0. Scope Lock
This system is a purpose-built appliance platform that boots directly into a single sealed Cartridge. It is not a general-purpose OS, not a desktop, and not a multi-user environment. It refuses to solve broad application compatibility and instead optimizes for integrity, repeatability, and deterministic single-task execution.

## 1. Core Philosophy and Invariants
- Verification always precedes execution.
- The kernel/host is a mechanism, never a decision-maker.
- Authority exists only in user space.
- Artifacts (kernel, drivers, cartridges) are immutable once sealed.
- A cartridge is untrusted until verified.
- No in-place mutation of sealed artifacts.

## 2. Authority Model
- Human tooling: defines intent and builds artifacts; cannot bypass runtime verification.
- Bootloader: verifies kernel image (signature/hash) and hands off control; no semantics.
- Kernel (micro-core): enforces boundaries and mediates access; never interprets intent.
- Driver modules (userspace): own specific hardware interfaces; cannot alter kernel policy.
- Application cartridges: request resources via declared capabilities only.

## 3. Trust Boundaries
- Bootloader <-> Kernel: only a signed/hashed kernel crosses.
- Kernel <-> Driver modules: only verified driver artifacts cross; no implicit trust.
- Kernel/Driver <-> Application cartridge: capability-gated channels only.
- Human <-> Tooling: all outputs must be sealed before acceptance.

## 4. Artifact Definition (Abstract)
An Artifact is any executable unit: kernel, driver, or application cartridge.
- Contains: header, identity metadata (Tanka), payload, integrity proof.
- Must never contain: mutable state that changes identity.
- Identity equivalence: artifacts are identical if payload hash matches.
- Invalidated by any byte-level change or unmet capability contract.

## 5. Artifact Header (Concrete Requirements)
- Magic number: 0xCAFEBABE
- Version: 16-bit semantic versioning for loader compatibility
- Entry point: 32/64-bit offset into payload
- Capability flags: declared hardware/resource access
- Payload hash: SHA-256 over payload bytes
- Header checksum: optional fast pre-check before full hash

## 6. Operational Metadata (Tanka)
- 5-7-5-7-7 syllable structure stored in artifact metadata.
- Non-authoritative: never used by kernel for logic.
- Used for human-facing auditability and as cryptographic seed material.

## 7. Lifecycle States
- Draft: editable, unsigned, non-executable.
- Sealed: immutable, hashed/signed, eligible for verification.
- Verified: integrity confirmed, eligible to execute.
- Executing: active, running module.
- Revoked: explicitly disallowed from execution.

## 8. Execution Model
- Boot chain performs verification at each step.
- Control transfers from host to a single cartridge entry point.
- Host remains resident for enforcement and mediation.
- Single active cartridge at a time.

## 9. Isolation and IPC
- Default: no direct hardware access for cartridges.
- Capability gating: hardware access granted only via declared capabilities.
- IPC via host-managed channels.
- Zero-copy shared memory windows for high-throughput paths (video/audio).

## 10. Failure Semantics
- Cartridge crash: terminate cartridge, return to switcher or reboot.
- Boundary violation: immediate halt and artifact revocation.
- Integrity failure: quarantine image and refuse execution.

## 11. Configuration Philosophy
- Configuration occurs at build/seal time.
- Runtime configuration limited to non-identity-affecting settings.
- Intent is human-facing and non-authoritative.

## 12. Non-Goals
- No general application compatibility layer.
- No background service ecosystem.
- No traditional package manager or rolling updates.
- No multi-user session model.

## 13. Success Metrics
- Verified boot: 100% success for valid artifacts.
- Tamper detection: any byte change prevents execution.
- Boot-to-module target: < 5 seconds on reference hardware.
- Recovery: reboot restores known-good state without manual intervention.

