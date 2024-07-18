pub fn multiplier(a: u32, b: u32) -> u32 {
    let mut result: u32 = 0;
    for i in 0..u32::BITS {
        let mask: u32 = 1 << i;
        let b_bit = if b & mask > 0 { 1 } else { 0 };
        if b_bit == 1 {
            result |= a << i;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_multiply_two_numbers() {
        let result = multiplier(2, 2);

        assert_eq!(result, 4);
    }

    #[test]
    fn can_multiply_zeroes() {
        let result = multiplier(0, 0);

        assert_eq!(result, 0);
    }


    #[test]
    fn can_multiply_big_values() {
        let result = multiplier(u32::MAX / 2, 2);

        // Minus one because of rounding for division
        assert_eq!(result, u32::MAX - 1);
    }
}
