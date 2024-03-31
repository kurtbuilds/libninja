use std::ops::Deref;
use std::sync::OnceLock;

use regex_lite::Regex;

pub fn sanitize(s: &str) -> String {
    let r = if s.contains('(') {
        static NO_PAREN: OnceLock<Regex> = OnceLock::new();
        let re = NO_PAREN.get_or_init(|| Regex::new(r"\([^)]*\)").unwrap());
        re.replace_all(s, "")
    } else {
        s.into()
    };
    let s = r.trim().to_string();
    if s.chars().next().unwrap().is_numeric() {
        format!("_{}", s)
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sanitize() {
        assert_eq!(sanitize("coupon type (foo)"), "coupon type");
    }
}
