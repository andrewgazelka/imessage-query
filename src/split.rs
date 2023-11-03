pub fn split_by(input: &[u8], separator: &[u8]) -> Vec<Vec<u8>> {
    assert!(!separator.is_empty(), "separator cannot be empty");

    let mut result = Vec::new();

    let mut start = 0;
    while let Some(end) = input[start..]
        .windows(separator.len())
        .position(|window| window == separator)
    {
        result.push(input[start..start + end].to_vec());
        start += end + separator.len();
    }

    result.push(input[start..].to_vec());

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_split_by() {
        let input = b"hello world";
        let separator = b" ";
        let expected = vec![b"hello", b"world"];
        let result = super::split_by(input, separator);
        assert_eq!(result, expected);
    }

    #[test]
    #[should_panic(expected = "separator cannot be empty")]
    fn test_split_by_empty() {
        let input = b"hello world";
        let separator = b"";
        super::split_by(input, separator);
    }

    #[test]
    fn test_split_by_empty_input() {
        let input = b"";
        let separator = b" ";
        let expected = vec![b""];
        let result = super::split_by(input, separator);
        assert_eq!(result, expected);
    }
}
