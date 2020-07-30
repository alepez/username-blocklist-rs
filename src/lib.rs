pub const BLOCKLIST: [&'static str; 538] = include!("list.json");

pub fn is_valid(s: &str) -> bool {
    !BLOCKLIST.contains(&s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_return_true() {
        assert!(is_valid("paperinik"));
    }

    #[test]
    fn is_valid_return_false() {
        assert!(!is_valid("root"));
    }
}
