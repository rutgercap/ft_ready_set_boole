
pub const ORDER: u16 = 2;

pub fn map(x: u16, y: u16) -> f64 {
    fn xy_to_hilbert_index(order: u16, x: u16, y: u16) -> u16 {
        let mut index = 0;
        let mut x = x;
        let mut y = y;
        let mut n = 1 << (order - 1);

        for _ in 0..order {
            let rx = (x & n) > 0;
            let ry = (y & n) > 0;

            index += n * n * ((3 * rx as u16) ^ ry as u16);

            if ry == false {
                if rx == true {
                    x = n - 1 - x;
                    y = n - 1 - y;
                }
                std::mem::swap(&mut x, &mut y);
            }

            n >>= 1;
        }

        index
    }

    fn hilbert_normalize(order: u16, x: u16, y: u16) -> f64 {
        let hilbert_index = xy_to_hilbert_index(order, x, y);
        let max_index = (1 << (2 * order)) - 1;
        hilbert_index as f64 / max_index as f64
    }

    hilbert_normalize(ORDER, x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        assert_eq!(map(0, 0), 0.0);
    }

    #[test]
    fn test_map_with_something() {
        let result = map(1, 1);
        assert!(result >= 0.0 && result <= 1.0);
    }

    #[test]
    fn test_map_with_big_number() {
        for i in 0..u16::MAX {
            let result = map(i, i);
            assert!(result >= 0.0 && result <= 1.0);
        }
    }
}
