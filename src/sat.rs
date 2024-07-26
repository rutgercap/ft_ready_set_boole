use crate::operator::Operator;

pub fn sat(formula: &str) -> bool {
    let operator = Operator::from_formula(formula);
    if operator.is_none() {
        return true;
    }
    let operator = operator.unwrap();
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_clause_is_satisfiable() {
        assert_eq!(sat(""), false);
    }

    #[test]
    fn test_sat() {
        assert_eq!(sat("AB|"), true);
    }
}