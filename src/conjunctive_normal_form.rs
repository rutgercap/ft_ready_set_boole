use itertools::Itertools;

use crate::operator::Operator;

impl Operator {
    pub fn to_conjunctive_normal_form(self) -> Operator {
        match self {
            Operator::Operand(_) => self,
            Operator::Not(a) => Operator::not(a.to_conjunctive_normal_form()),
            Operator::And(a, b) => {
                let a = a.to_conjunctive_normal_form();
                let b = b.to_conjunctive_normal_form();
                Operator::And(Box::new(a), Box::new(b))
            }
            Operator::Or(a, b) => {
                let a = a.to_conjunctive_normal_form();
                let b = b.to_conjunctive_normal_form();
                match (a, b) {
                    (Operator::And(c, d), e) => Operator::and(
                        Operator::or(*c, e.clone()).to_conjunctive_normal_form(),
                        Operator::or(*d, e).to_conjunctive_normal_form(),
                    ),
                    (e, Operator::And(c, d)) => Operator::and(
                        Operator::Or(Box::new(e.clone()), c.clone()).to_conjunctive_normal_form(),
                        Operator::Or(Box::new(e), d.clone()).to_conjunctive_normal_form(),
                    ),
                    (e, f) => Operator::Or(Box::new(e), Box::new(f)),
                }
            }
            a => panic!("Operator {:?} not allowed. Expression must be in NNF", a),
        }
    }
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let operands = Operator::from_formula(formula);
    if operands.is_none() {
        return String::new();
    }
    let mut result = operands
        .unwrap()
        .to_negation_normal_form()
        .to_conjunctive_normal_form().to_string();
    let n = result.chars().filter(|c| *c == '&').count();
    result = result.chars().filter(|c| *c != '&').join("");
    result.push_str(&"&".repeat(n));
    result
}



#[cfg(test)]
mod tests {

    use crate::truth_table::tests::truth_tables_equal;

    use super::*;

    #[test]
    fn conjunctive_normal_form_works_with_empty_string() {
        let result = conjunctive_normal_form("");

        assert_eq!(result, "");
    }

    #[test]
    fn conjunctive_normal_form_works_with_negated_and() {
        let original = "AB&!";
        let expected = "A!B!|";
        let result = conjunctive_normal_form(&original);

        assert_eq!(result, expected);
        truth_tables_equal(original, &result);
    }

    #[test]
    fn conjunctive_normal_form_works_with_negated_or() {
        let original = "AB|!";
        let expected = "A!B!&";
        let result = conjunctive_normal_form(&original);

        assert_eq!(result, expected);
        truth_tables_equal(original, &result);
    }

    #[test]
    fn conjunctive_normal_form_works_with_deep_and() {
        let original = "AB|C&D&";
        let expected = "AB|CD&&";
        let result = conjunctive_normal_form(&original);

        assert_eq!(result, expected);
        truth_tables_equal(original, &result);
    }

    #[test]
    fn conjunctive_normal_form_works_with_1() {
        let original = "ABCD&|&";
        let expected = "ABC|BD|&&";
        let result = conjunctive_normal_form(&original);

        assert_eq!(result, expected);
        truth_tables_equal(original, &result);
    }

    #[test]
    fn conjunctive_normal_form_works_with_many_ors() {
        let original = "AB|C|D|";
        let expected = "AB|C|D|";
        let result = conjunctive_normal_form(&original);

        assert_eq!(result, expected);
        truth_tables_equal(original, &result);
    }

    #[test]
    fn conjunctive_normal_form_works_with_many_ands() {
        let original = "AB&C&D&";
        let expected = "ABCD&&&";
        let result = conjunctive_normal_form(&original);

        assert_eq!(result, expected);
        truth_tables_equal(original, &result);
    }

    #[test]
    fn conjunctive_normal_form_works_complicated() {
        let original = "AB&!C!|";
        let expected = "A!B!|C!|";
        let result = conjunctive_normal_form(&original);

        assert_eq!(result, expected);
        truth_tables_equal(original, &result);
    }

    #[test]
    fn last_subject_test() {
        let original = "AB|!C!&";
        let expected = "A!B!C!&&";
        let result = conjunctive_normal_form(&original);

        truth_tables_equal(original, &expected);
        truth_tables_equal(original, &result);
        assert_eq!(result, expected);
    }
}
