//! Capability definitions
//!
//! Capabilities control what hardware and resources an artifact can access.

use core::fmt;

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::vec::Vec;

/// Capability flags (bitfield in artifact header)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Capability(pub u64);

impl Capability {
    // Hardware capabilities
    pub const NONE: u64 = 0;
    pub const GPU: u64 = 1 << 0;
    pub const AUDIO: u64 = 1 << 1;
    pub const INPUT: u64 = 1 << 2;
    pub const NETWORK: u64 = 1 << 3;
    pub const STORAGE: u64 = 1 << 4;
    pub const USB: u64 = 1 << 5;

    // IPC capabilities
    pub const IPC_BASIC: u64 = 1 << 16;
    pub const IPC_SHARED_MEMORY: u64 = 1 << 17;

    // System capabilities
    pub const REBOOT: u64 = 1 << 32;

    pub fn new(flags: u64) -> Self {
        Self(flags)
    }

    pub fn has(&self, cap: u64) -> bool {
        (self.0 & cap) == cap
    }

    pub fn grant(&mut self, cap: u64) {
        self.0 |= cap;
    }

    pub fn revoke(&mut self, cap: u64) {
        self.0 &= !cap;
    }

    pub fn is_subset_of(&self, other: &Capability) -> bool {
        (self.0 & other.0) == self.0
    }
}

impl fmt::Display for Capability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut caps = Vec::new();

        if self.has(Self::GPU) { caps.push("GPU"); }
        if self.has(Self::AUDIO) { caps.push("AUDIO"); }
        if self.has(Self::INPUT) { caps.push("INPUT"); }
        if self.has(Self::NETWORK) { caps.push("NETWORK"); }
        if self.has(Self::STORAGE) { caps.push("STORAGE"); }
        if self.has(Self::USB) { caps.push("USB"); }
        if self.has(Self::IPC_BASIC) { caps.push("IPC_BASIC"); }
        if self.has(Self::IPC_SHARED_MEMORY) { caps.push("IPC_SHARED_MEMORY"); }
        if self.has(Self::REBOOT) { caps.push("REBOOT"); }

        if caps.is_empty() {
            write!(f, "NONE")
        } else {
            write!(f, "{}", caps.join(" | "))
        }
    }
}
