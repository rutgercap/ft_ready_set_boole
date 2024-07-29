use crate::curve::ORDER;

pub fn reverse_map(n: f64) -> (u16, u16) {
    fn hilbert_index_to_xy(order: u16, index: u16) -> (u16, u16) {
        let mut x = 0;
        let mut y = 0;
        let mut t = index;
        let mut n = 1 << (order - 1);

        for _ in 0..order {
            let rx = (t >> 1) & 1;
            let ry = t & 1;

            if ry == 0 {
                if rx == 1 {
                    x = n - 1 - x;
                    y = n - 1 - y;
                }
                std::mem::swap(&mut x, &mut y);
            }

            x += n * rx;
            y += n * ry;
            t >>= 2;
            n >>= 1;
        }

        (x, y)
    }

    fn normalize_to_hilbert_index(order: u16, normalized_value: f64) -> u16 {
        let max_index = (1 << (2 * order)) - 1;
        (normalized_value * max_index as f64).round() as u16
    }

    let index = normalize_to_hilbert_index(ORDER, n);
    hilbert_index_to_xy(ORDER, index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curve::map;

    #[test]
    fn test_reverse_map() {
        let original = (0, 0);
        let result = map(original.0, original.1);
        assert_eq!(reverse_map(result), original);
    }

    #[test]
    fn test_reverse_map_with_big_number() {
        let original = (10, 10);
        let result = map(original.0, original.1);
        assert_eq!(reverse_map(result), original);
    }
}
