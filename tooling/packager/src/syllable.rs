//! Syllable counting for Tanka validation
//!
//! Simple English syllable counter based on vowel groups.
//! Not perfect, but sufficient for validation purposes.

/// Count syllables in a word (English approximation)
pub fn count_syllables(word: &str) -> usize {
    let word = word.to_lowercase();
    let word = word.trim();

    if word.is_empty() {
        return 0;
    }

    // Special cases
    match word {
        "the" | "a" | "an" => return 1,
        _ => {}
    }

    let vowels = ['a', 'e', 'i', 'o', 'u', 'y'];
    let mut count = 0;
    let mut previous_was_vowel = false;

    let chars: Vec<char> = word.chars().collect();

    for (i, &c) in chars.iter().enumerate() {
        let is_vowel = vowels.contains(&c);

        if is_vowel && !previous_was_vowel {
            count += 1;
        }

        previous_was_vowel = is_vowel;
    }

    // Silent 'e' at end
    if word.ends_with('e') && count > 1 {
        count -= 1;
    }

    // Handle words ending in 'le' after consonant
    if word.len() > 2 && word.ends_with("le") {
        let before_le = chars[chars.len() - 3];
        if !vowels.contains(&before_le) {
            count += 1;
        }
    }

    // Minimum 1 syllable per word
    count.max(1)
}

/// Count syllables in a line
pub fn count_line_syllables(line: &str) -> usize {
    line.split_whitespace()
        .map(|word| {
            // Remove punctuation
            let word: String = word.chars()
                .filter(|c| c.is_alphabetic() || *c == '\'')
                .collect();
            count_syllables(&word)
        })
        .sum()
}

/// Validate Tanka syllable pattern (5-7-5-7-7)
pub fn validate_tanka(lines: &[String; 5]) -> Result<(), TankaError> {
    const EXPECTED: [usize; 5] = [5, 7, 5, 7, 7];

    for (i, line) in lines.iter().enumerate() {
        let count = count_line_syllables(line);
        let expected = EXPECTED[i];

        if count != expected {
            return Err(TankaError::InvalidSyllableCount {
                line_number: i + 1,
                expected,
                actual: count,
                line: line.clone(),
            });
        }
    }

    Ok(())
}

#[derive(Debug)]
pub enum TankaError {
    InvalidSyllableCount {
        line_number: usize,
        expected: usize,
        actual: usize,
        line: String,
    },
}

impl std::fmt::Display for TankaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TankaError::InvalidSyllableCount {
                line_number,
                expected,
                actual,
                line,
            } => {
                write!(
                    f,
                    "Line {}: Expected {} syllables, got {} in \"{}\"",
                    line_number, expected, actual, line
                )
            }
        }
    }
}

impl std::error::Error for TankaError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_syllable_counting() {
        assert_eq!(count_syllables("unnamed"), 3);
        assert_eq!(count_syllables("artifact"), 3);
        assert_eq!(count_syllables("identity"), 4);
        assert_eq!(count_syllables("awaits"), 2);
        assert_eq!(count_syllables("silent"), 2);
        assert_eq!(count_syllables("bytes"), 1);
    }

    #[test]
    fn test_line_counting() {
        assert_eq!(count_line_syllables("Unnamed artifact"), 6);
        assert_eq!(count_line_syllables("Identity awaits a name"), 7);
        assert_eq!(count_line_syllables("In silent bytes"), 5);
    }

    #[test]
    fn test_valid_tanka() {
        let tanka = [
            "Unnamed artifact".to_string(),
            "Identity awaits a name".to_string(),
            "In silent bytes".to_string(),
            "Potential lies encrypted".to_string(),
            "Purpose not yet revealed".to_string(),
        ];

        assert!(validate_tanka(&tanka).is_ok());
    }
}
