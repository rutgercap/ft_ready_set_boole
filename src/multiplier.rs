pub fn multiplier(a: u32, b: u32) -> u32 {

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_multiply_two_numbers() {
        let result = multiplier(2, 2);
        
        assert_eq!(result, 4);
    }
}
