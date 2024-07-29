use crate::{operator::Operator, truth_table::truth_table};

fn operands_in_formula(formula: &str) -> Vec<char> {
    formula
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(vec![], |mut acc, c| {
            if !acc.contains(&c) {
                acc.push(c);
            }
            acc
        })
}

pub fn sat(formula: &str) -> bool {
    let operator = Operator::from_formula(formula);
    if operator.is_none() {
        return true;
    }
    let operator = operator.unwrap();
    let operands = operands_in_formula(formula);
    let table = truth_table(&operator, &operands);
    table.iter().any(|(_, b)| *b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_clause_is_satisfiable() {
        assert_eq!(sat(""), true);
    }

    #[test]
    fn test_sat() {
        assert_eq!(sat("AB|"), true);
    }

    #[test]
    fn sat_returns_false_for_unstatisfiable_formula() {
        assert_eq!(sat("AA!&"), false);
    }

    #[test]
    fn sat_works_with_other_operators() {
        assert_eq!(sat("AA^"), false);
    }
}
