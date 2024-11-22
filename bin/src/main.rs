use lib::some_function;

fn main() {
    // cargo run --release and cargo run fail
    assert_eq!(some_function(), "bar".to_string());
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
