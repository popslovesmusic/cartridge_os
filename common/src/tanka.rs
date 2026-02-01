//! Tanka metadata (5-7-5-7-7 syllable structure)
//!
//! Non-authoritative human-facing metadata stored in artifact.

/// Tanka structure: 5 lines with syllable counts [5, 7, 5, 7, 7]
/// Stored as fixed-size buffers for no_std compatibility
#[derive(Debug, Clone, Copy)]
pub struct Tanka {
    /// Fixed 320-byte buffer (5 lines × 64 bytes)
    data: [u8; 320],
}

impl Tanka {
    /// Maximum line length in bytes
    pub const MAX_LINE_LENGTH: usize = 64;

    /// Expected syllable counts
    pub const SYLLABLE_PATTERN: [usize; 5] = [5, 7, 5, 7, 7];

    /// Create from raw byte buffer
    pub const fn from_bytes(data: [u8; 320]) -> Self {
        Self { data }
    }

    /// Get raw byte buffer
    pub const fn to_bytes(&self) -> &[u8; 320] {
        &self.data
    }

    /// Get a specific line (returns slice of buffer)
    pub fn get_line(&self, index: usize) -> Option<&[u8]> {
        if index >= 5 {
            return None;
        }

        let start = index * Self::MAX_LINE_LENGTH;
        let end = start + Self::MAX_LINE_LENGTH;
        let line_bytes = &self.data[start..end];

        // Find null terminator or end
        let len = line_bytes.iter().position(|&b| b == 0).unwrap_or(Self::MAX_LINE_LENGTH);

        Some(&line_bytes[..len])
    }

    /// Validate basic structure (at least some non-zero data)
    pub fn is_valid(&self) -> bool {
        self.data.iter().any(|&b| b != 0)
    }
}

impl Default for Tanka {
    fn default() -> Self {
        let mut data = [0u8; 320];

        // Default Tanka text (const-compatible)
        const LINES: [&[u8]; 5] = [
            b"Unnamed artifact",
            b"Identity awaits a name",
            b"In silent bytes",
            b"Potential lies encrypted",
            b"Purpose not yet revealed",
        ];

        for (i, line) in LINES.iter().enumerate() {
            let start = i * Self::MAX_LINE_LENGTH;
            let len = line.len().min(Self::MAX_LINE_LENGTH);
            data[start..start + len].copy_from_slice(&line[..len]);
        }

        Self { data }
    }
}
