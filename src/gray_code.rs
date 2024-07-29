pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gray_code_0(value: u32, expected: u32) {
        let result = gray_code(value);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_gray_code_0() {
        let test_cases = [
            (0, 0),
            (1, 1),
            (2, 3),
            (3, 2),
            (4, 6),
            (5, 7),
            (6, 5),
            (7, 4),
            (8, 12),
            (2147483648, 3221225472),
        ];

        for (value, expected) in test_cases {
            gray_code_0(value, expected);
        }
    }
}
