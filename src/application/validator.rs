/// Validates whether a solution matches the target content
///
/// This encapsulates the validation logic with normalization rules.
pub struct SolutionValidator;

impl SolutionValidator {
    pub fn new() -> Self {
        Self
    }

    /// Validates if the actual content matches the expected content
    ///
    /// Normalization rules:
    /// - Trim each line
    /// - Remove empty lines
    /// - Join with newlines for comparison
    pub fn is_valid(&self, actual: &str, expected: &str) -> bool {
        let normalized_actual = self.normalize(actual);
        let normalized_expected = self.normalize(expected);
        normalized_actual == normalized_expected
    }

    fn normalize(&self, content: &str) -> String {
        content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Default for SolutionValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exact_match() {
        let validator = SolutionValidator::new();
        assert!(validator.is_valid("Hello world", "Hello world"));
    }

    #[test]
    fn test_whitespace_normalization() {
        let validator = SolutionValidator::new();
        assert!(validator.is_valid("  Hello world  ", "Hello world"));
        assert!(validator.is_valid("Hello world\n\n", "Hello world"));
    }

    #[test]
    fn test_mismatch() {
        let validator = SolutionValidator::new();
        assert!(!validator.is_valid("Hello REMOVE world", "Hello world"));
    }
}
