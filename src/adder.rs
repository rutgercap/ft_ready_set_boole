pub fn add(left: usize, right: usize) -> usize {
    let mut result: usize = 0;
    let mut carry = 0;
    for i in 0..usize::BITS {
        let mask: usize = 1 << i;
        let current_left_bit = if left & mask > 0 { 1 } else { 0 };
        let current_right_bit = if right & mask > 0 { 1 } else { 0 };
        let sum_bit = current_left_bit ^ current_right_bit ^ carry;
        carry = (current_left_bit & current_right_bit) | (current_left_bit & carry) | (current_right_bit & carry);
        result |= sum_bit << i;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_add_to_numbers() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn adding_works_with_zero() {
        let result = add(2, 0);
        assert_eq!(result, 2);
    }

    #[test]
    fn adding_works_with_zeros() {
        let result = add(0, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn adding_works_with_max_values() {
        let result = add(usize::MAX, 0);
        assert_eq!(result, usize::MAX);
    }
}
