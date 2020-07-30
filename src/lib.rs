/// The blocklist
pub const BLOCKLIST: [&'static str; 538] = include!("list.json");

/// Check if the word is in the blacklist, return false (validation failed), otherwise true.
pub fn validate(s: &str) -> bool {
    !BLOCKLIST.contains(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_return_true() {
        assert!(validate("paperinik"));
    }

    #[test]
    fn is_valid_return_false() {
        assert!(!validate("root"));
    }
}
