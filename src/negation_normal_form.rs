use crate::operator::Operator;

impl Operator {
    pub fn to_negation_normal_form(self) -> Operator {
        match self {
            Operator::Not(c) => match *c {
                Operator::Operand(_) => Operator::not(c.to_negation_normal_form()),
                Operator::Not(d) => d.to_negation_normal_form(),
                Operator::And(a, b) => Operator::with_two(
                    '|',
                    Operator::not(*a).to_negation_normal_form(),
                    Operator::not(*b).to_negation_normal_form(),
                ),
                Operator::Or(a, b) => Operator::with_two(
                    '&',
                    Operator::not(*a).to_negation_normal_form(),
                    Operator::not(*b).to_negation_normal_form(),
                ),
                Operator::Implies(a, b) => Operator::with_two(
                    '|',
                    a.to_negation_normal_form(),
                    Operator::not(b.to_negation_normal_form()),
                ),
                Operator::Xor(a, b) => Operator::with_two(
                    '=',
                    a.to_negation_normal_form(),
                    b.to_negation_normal_form(),
                )
                .to_negation_normal_form(),
                Operator::Equals(a, b) => Operator::with_two(
                    '^',
                    a.to_negation_normal_form(),
                    b.to_negation_normal_form(),
                )
                .to_negation_normal_form(),
            },
            Operator::Equals(a, b) => Operator::with_two(
                '|',
                Operator::with_two(
                    '&',
                    a.clone().to_negation_normal_form(),
                    b.clone().to_negation_normal_form(),
                ),
                Operator::with_two(
                    '&',
                    Operator::not(a.to_negation_normal_form()),
                    Operator::not(b.to_negation_normal_form()),
                ),
            ),
            Operator::Xor(a, b) => Operator::with_two(
                '|',
                Operator::with_two(
                    '&',
                    Operator::not(a.clone().to_negation_normal_form()),
                    b.clone().to_negation_normal_form(),
                ),
                Operator::with_two(
                    '&',
                    a.to_negation_normal_form(),
                    Operator::not(b.to_negation_normal_form()),
                ),
            ),
            Operator::Implies(a, b) => Operator::with_two(
                '|',
                Operator::not(a.to_negation_normal_form()),
                b.to_negation_normal_form(),
            ),
            Operator::And(a, b) => Operator::with_two(
                '&',
                a.to_negation_normal_form(),
                b.to_negation_normal_form(),
            ),
            Operator::Or(a, b) => Operator::with_two(
                '|',
                a.to_negation_normal_form(),
                b.to_negation_normal_form(),
            ),
            Operator::Operand(_) => self,
        }
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    let operator = Operator::from_formula(formula);
    if operator.is_none() {
        return "".to_string();
    }
    let nnf = operator.unwrap().to_negation_normal_form();
    nnf.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negation_normal_form_works_with_empty_string() {
        let result = negation_normal_form("");

        assert_eq!(result, "");
    }

    #[test]
    fn negation_normal_form_works_with_and() {
        let result = negation_normal_form("AB&");

        assert_eq!(result, "AB&");
    }

    #[test]
    fn negation_normal_form_works_when_negating_conjunction() {
        let result = negation_normal_form("AB&!");

        assert_eq!(result, "A!B!|");
    }

    #[test]
    fn negation_normal_form_works_with_deep_and() {
        let result = negation_normal_form("AB&BC&|!");

        assert_eq!(result, "A!B!|B!C!|&");
    }

    #[test]
    fn negation_normal_form_works_with_deep_negation() {
        let result = negation_normal_form("AB&!AB&&");

        assert_eq!(result, "A!B!|AB&&");
    }

    #[test]
    fn negation_normal_form_works_with_or() {
        let result = negation_normal_form("AB|");

        assert_eq!(result, "AB|");
    }

    #[test]
    fn negation_normal_form_works_with_negation() {
        let result = negation_normal_form("A!");
        assert_eq!(result, "A!");
    }

    #[test]
    fn negation_normal_form_works_with_double_negation() {
        let result = negation_normal_form("A!!");

        assert_eq!(result, "A");
    }

    #[test]
    fn negation_normal_form_works_with_triple_negation() {
        let result = negation_normal_form("A!!!");

        assert_eq!(result, "A!");
    }

    #[test]
    fn negation_normal_form_works_when_negating_disjunction() {
        let result = negation_normal_form("AB|!");

        assert_eq!(result, "A!B!&");
    }

    #[test]
    fn negation_normal_form_works_with_equals() {
        let result = negation_normal_form("AB=");

        assert_eq!(result, "AB&A!B!&|");
    }

    #[test]
    fn negation_normal_form_works_with_negated_equals() {
        let result = negation_normal_form("AB=!");

        assert_eq!(result, "A!B&AB!&|");
    }

    #[test]
    fn negation_normal_form_works_with_xor() {
        let result = negation_normal_form("AB^");

        assert_eq!(result, "A!B&AB!&|");
    }

    #[test]
    fn negation_normal_form_works_with_negated_xor() {
        let result = negation_normal_form("AB^!");

        assert_eq!(result, "AB&A!B!&|");
    }

    #[test]
    fn negation_normal_form_works_with_implies() {
        let result = negation_normal_form("AB>");

        assert_eq!(result, "A!B|");
    }

    #[test]
    fn negation_normal_form_works_with_negated_implies() {
        let result = negation_normal_form("AB>!");

        assert_eq!(result, "AB!|");
    }

    #[test]
    fn negation_normal_form_works_with_complicated_things() {
        let operators = Operator::from_formula("AB|C&!").unwrap();
        let expected_operator_tree = Operator::with_two(
            '|',
            Operator::with_two(
                '&',
                Operator::not(Operator::operand('A')),
                Operator::not(Operator::operand('B')),
            ),
            Operator::not(Operator::operand('C')),
        );
        assert_eq!(operators.to_negation_normal_form(), expected_operator_tree);
        // assert_eq!(result, "A!B!&C!|");
    }
}
