pub fn some_function() -> String {
    #[cfg(any(test, feature = "return_foo"))]
    return "foo".to_string();
    #[cfg(not(any(test, feature = "return_foo")))]
    return "bar".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_function() {
        // cargo test passes
        assert_eq!(some_function(), "foo".to_string());
    }
}
