//! Artifact format definitions
//!
//! Defines the header structure and metadata for all executable artifacts:
//! kernels, drivers, and application cartridges.

use sha2::{Digest, Sha256};

#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

#[cfg(feature = "std")]
use std::vec::Vec;

/// Magic number identifying a valid artifact: 0xCAFEBABE
pub const ARTIFACT_MAGIC: u32 = 0xCAFE_BABE;

/// Current artifact format version
pub const ARTIFACT_VERSION: u16 = 1;

/// Type of artifact
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ArtifactType {
    Kernel = 0,
    Driver = 1,
    Cartridge = 2,
}

/// Artifact header (fixed 128 bytes)
///
/// This header precedes every executable artifact and contains
/// all information needed for verification and loading.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ArtifactHeader {
    /// Magic number (0xCAFEBABE)
    pub magic: u32,

    /// Format version
    pub version: u16,

    /// Type of artifact
    pub artifact_type: u8,

    /// Reserved for alignment
    pub _reserved: u8,

    /// Entry point offset from payload start
    pub entry_offset: u64,

    /// Capability flags (bitfield)
    pub capabilities: u64,

    /// Payload size in bytes
    pub payload_size: u64,

    /// SHA-256 hash of payload
    pub payload_hash: [u8; 32],

    /// Optional header checksum (CRC32)
    pub header_checksum: u32,

    /// Padding to 128 bytes
    pub _padding: [u8; 36],
}

impl ArtifactHeader {
    /// Size of the header in bytes
    pub const SIZE: usize = 128;

    /// Create a new artifact header
    pub fn new(
        artifact_type: ArtifactType,
        entry_offset: u64,
        capabilities: u64,
        payload: &[u8],
    ) -> Self {
        let mut header = Self {
            magic: ARTIFACT_MAGIC,
            version: ARTIFACT_VERSION,
            artifact_type: artifact_type as u8,
            _reserved: 0,
            entry_offset,
            capabilities,
            payload_size: payload.len() as u64,
            payload_hash: [0u8; 32],
            header_checksum: 0,
            _padding: [0u8; 36],
        };

        // Compute payload hash
        let mut hasher = Sha256::new();
        hasher.update(payload);
        header.payload_hash.copy_from_slice(&hasher.finalize());

        // Compute header checksum
        header.header_checksum = header.compute_checksum();

        header
    }

    /// Verify header magic and version
    pub fn is_valid(&self) -> bool {
        self.magic == ARTIFACT_MAGIC && self.version == ARTIFACT_VERSION
    }

    /// Compute CRC32 checksum over header (excluding checksum field)
    fn compute_checksum(&self) -> u32 {
        // TODO: Implement CRC32 calculation
        // For now, return dummy value
        0xDEADBEEF
    }

    /// Verify payload hash matches header
    pub fn verify_payload(&self, payload: &[u8]) -> bool {
        if payload.len() as u64 != self.payload_size {
            return false;
        }

        let mut hasher = Sha256::new();
        hasher.update(payload);
        let computed_hash = hasher.finalize();

        &computed_hash[..] == &self.payload_hash[..]
    }
}

/// Complete artifact (header + payload)
pub struct Artifact {
    pub header: ArtifactHeader,
    pub payload: Vec<u8>,
}

impl Artifact {
    /// Load and verify an artifact from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self, ArtifactError> {
        if data.len() < ArtifactHeader::SIZE {
            return Err(ArtifactError::TooSmall);
        }

        // Parse header
        let header = unsafe {
            core::ptr::read(data.as_ptr() as *const ArtifactHeader)
        };

        if !header.is_valid() {
            return Err(ArtifactError::InvalidHeader);
        }

        // Extract payload
        let payload_start = ArtifactHeader::SIZE;
        let payload_end = payload_start + header.payload_size as usize;

        if data.len() < payload_end {
            return Err(ArtifactError::TruncatedPayload);
        }

        let payload = data[payload_start..payload_end].to_vec();

        // Verify payload hash
        if !header.verify_payload(&payload) {
            return Err(ArtifactError::HashMismatch);
        }

        Ok(Self { header, payload })
    }

    /// Serialize artifact to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(ArtifactHeader::SIZE + self.payload.len());

        // Write header
        let header_bytes = unsafe {
            core::slice::from_raw_parts(
                &self.header as *const ArtifactHeader as *const u8,
                ArtifactHeader::SIZE,
            )
        };
        bytes.extend_from_slice(header_bytes);

        // Write payload
        bytes.extend_from_slice(&self.payload);

        bytes
    }
}

#[derive(Debug)]
pub enum ArtifactError {
    TooSmall,
    InvalidHeader,
    TruncatedPayload,
    HashMismatch,
}
