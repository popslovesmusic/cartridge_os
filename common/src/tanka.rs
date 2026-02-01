//! Tanka metadata (5-7-5-7-7 syllable structure)
//!
//! Non-authoritative human-facing metadata stored in artifact.

/// Tanka structure: 5 lines with syllable counts [5, 7, 5, 7, 7]
#[derive(Debug, Clone)]
pub struct Tanka {
    pub lines: [String; 5],
}

impl Tanka {
    /// Maximum line length in bytes
    pub const MAX_LINE_LENGTH: usize = 64;

    /// Expected syllable counts
    pub const SYLLABLE_PATTERN: [usize; 5] = [5, 7, 5, 7, 7];

    pub fn new(lines: [String; 5]) -> Self {
        Self { lines }
    }

    /// Validate line lengths (basic check)
    pub fn is_valid(&self) -> bool {
        self.lines.iter().all(|line| {
            !line.is_empty() && line.len() <= Self::MAX_LINE_LENGTH
        })
    }

    /// Serialize to fixed-size buffer (320 bytes: 5 lines × 64 bytes)
    pub fn to_bytes(&self) -> [u8; 320] {
        let mut buffer = [0u8; 320];

        for (i, line) in self.lines.iter().enumerate() {
            let start = i * Self::MAX_LINE_LENGTH;
            let line_bytes = line.as_bytes();
            let len = line_bytes.len().min(Self::MAX_LINE_LENGTH);
            buffer[start..start + len].copy_from_slice(&line_bytes[..len]);
        }

        buffer
    }

    /// Deserialize from fixed-size buffer
    pub fn from_bytes(buffer: &[u8; 320]) -> Self {
        let mut lines: [String; 5] = Default::default();

        for i in 0..5 {
            let start = i * Self::MAX_LINE_LENGTH;
            let end = start + Self::MAX_LINE_LENGTH;
            let line_bytes = &buffer[start..end];

            // Find null terminator or end
            let len = line_bytes.iter().position(|&b| b == 0).unwrap_or(Self::MAX_LINE_LENGTH);

            lines[i] = String::from_utf8_lossy(&line_bytes[..len]).to_string();
        }

        Self { lines }
    }
}

impl Default for Tanka {
    fn default() -> Self {
        Self {
            lines: [
                "Unnamed artifact".to_string(),
                "Identity awaits a name".to_string(),
                "In silent bytes".to_string(),
                "Potential lies encrypted".to_string(),
                "Purpose not yet revealed".to_string(),
            ],
        }
    }
}
