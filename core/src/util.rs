use std::borrow::Cow;

/// really dumb approximate attempt at making singular
pub fn singular(s: &str) -> Cow<'_, str> {
    if s.ends_with("ies") {
        let mut s = s[..s.len() - 3].to_string();
        s.push('y');
        Cow::Owned(s)
    } else if s.ends_with("es") {
        Cow::Owned(s[..s.len() - 2].to_string())
    } else if !s.ends_with("ss") && s.ends_with('s') {
        Cow::Borrowed(&s[..s.len() - 1])
    } else {
        Cow::Borrowed(s)
    }
}

pub fn is_plural(s: &str) -> bool {
    if s.ends_with("ies") {
        true
    } else if s.ends_with("es") {
        true
    } else if !s.ends_with("ss") && s.ends_with('s') {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_singular() {
        assert!(singular("cats") == "cat");
        assert!(singular("class") == "class");
        assert!(singular("parties") == "party");
        assert!(singular("party") == "party");
        assert!(singular("party") == "party");

        // assert!(singular("mice") == "mouse");
        // assert!(singular("alumni") == "alumnus");
    }

    #[test]
    fn test_is_plural() {
        assert!(is_plural("cats"));
        assert!(is_plural("parties"));
        assert!(is_plural("knives"));
        assert!(is_plural("potatoes"));
        assert!(!is_plural("cat"));
        assert!(!is_plural("class"));
        assert!(!is_plural("party"));
        // assert!(is_plural("alumni"));
        // assert!(is_plural("bacteria"));
    }
}
