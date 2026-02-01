//! Common types and utilities shared across the Cartridge OS
//!
//! This crate contains artifact format definitions, capability types,
//! and verification logic used by the kernel, bootloader, and tooling.

#![cfg_attr(not(feature = "std"), no_std)]

pub mod artifact;
pub mod capability;
pub mod tanka;
pub mod verification;

pub use artifact::{Artifact, ArtifactHeader, ArtifactType};
pub use capability::Capability;
pub use tanka::Tanka;
