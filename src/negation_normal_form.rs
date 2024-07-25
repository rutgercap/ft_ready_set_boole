use crate::operator::Operator;

fn formula_to_string(operator: &Operator) -> String {
    match operator {
        Operator::Not(c) => match c.as_ref() {
            Operator::Not(d) => format!("{}", formula_to_string(d)),
            Operator::And(a, b) => format!(
                "{}{}|",
                formula_to_string(&Operator::not(*a.clone())),
                formula_to_string(&Operator::not(*b.clone()))
            ),
            Operator::Or(a, b) => format!(
                "{}{}&",
                formula_to_string(&Operator::not(*a.clone())),
                formula_to_string(&Operator::not(*b.clone()))
            ),
            Operator::Operand(c) => format!("{}!", c),
            Operator::Xor(a, b) => {
                formula_to_string(&Operator::with_two('=', *a.clone(), *b.clone()))
            }
            Operator::Implies(a, b) => format!(
                "{}{}|",
                formula_to_string(a),
                formula_to_string(&Operator::not(*b.clone())),
            ),
            Operator::Equals(a, b) => {
                formula_to_string(&Operator::with_two('^', *a.clone(), *b.clone()))
            }
        },
        Operator::Operand(c) => c.to_string(),
        Operator::And(a, b) => format!("{}{}&", formula_to_string(a), formula_to_string(b)),
        Operator::Or(a, b) => format!("{}{}|", formula_to_string(a), formula_to_string(b)),
        Operator::Xor(a, b) => format!(
            "{}{}&{}{}&|",
            formula_to_string(&Operator::not(*a.clone())),
            formula_to_string(b),
            formula_to_string(a),
            formula_to_string(&Operator::not(*b.clone()))
        ),
        Operator::Implies(a, b) => format!(
            "{}{}|",
            formula_to_string(&Operator::not(*a.clone())),
            formula_to_string(b)
        ),
        Operator::Equals(a, b) => format!(
            "{}{}&{}{}&|",
            formula_to_string(a),
            formula_to_string(b),
            formula_to_string(&Operator::not(*a.clone())),
            formula_to_string(&Operator::not(*b.clone()))
        ),
    }
}

pub fn negation_normal_form(formula: &str) -> String {
    let operator = Operator::from_formula(formula);
    if operator.is_none() {
        return "".to_string();
    }
    formula_to_string(&operator.unwrap())   
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
        let result = formula_to_string(&Operator::from_formula("AB&").unwrap());

        assert_eq!(result, "AB&");
    }

    #[test]
    fn negation_normal_form_works_with_or() {
        let result = formula_to_string(&Operator::from_formula("AB|").unwrap());

        assert_eq!(result, "AB|");
    }

    #[test]
    fn negation_normal_form_works_with_negation() {
        let result = formula_to_string(&Operator::from_formula("A!").unwrap());

        assert_eq!(result, "A!");
    }

    #[test]
    fn negation_normal_form_works_with_double_negation() {
        let result = formula_to_string(&Operator::from_formula("A!!").unwrap());

        assert_eq!(result, "A");
    }

    #[test]
    fn negation_normal_form_works_with_triple_negation() {
        let result = formula_to_string(&Operator::from_formula("A!!!").unwrap());

        assert_eq!(result, "A!");
    }

    #[test]
    fn negation_normal_form_works_when_negating_conjunction() {
        let result = formula_to_string(&Operator::from_formula("AB&!").unwrap());

        assert_eq!(result, "A!B!|");
    }

    #[test]
    fn negation_normal_form_works_when_negating_disjunction() {
        let result = formula_to_string(&Operator::from_formula("AB|!").unwrap());

        assert_eq!(result, "A!B!&");
    }

    #[test]
    fn negation_normal_form_works_with_equals() {
        let result = formula_to_string(&Operator::from_formula("AB=").unwrap());

        assert_eq!(result, "AB&A!B!&|");
    }

    #[test]
    fn negation_normal_form_works_with_negated_equals() {
        let result = formula_to_string(&Operator::from_formula("AB=!").unwrap());

        assert_eq!(result, "A!B&AB!&|");
    }

    #[test]
    fn negation_normal_form_works_with_xor() {
        let result = formula_to_string(&Operator::from_formula("AB^").unwrap());

        assert_eq!(result, "A!B&AB!&|");
    }

    #[test]
    fn negation_normal_form_works_with_negated_xor() {
        let result = formula_to_string(&Operator::from_formula("AB^!").unwrap());

        assert_eq!(result, "AB&A!B!&|");
    }

    #[test]
    fn negation_normal_form_works_with_implies() {
        let result = formula_to_string(&Operator::from_formula("AB>").unwrap());

        assert_eq!(result, "A!B|");
    }

    #[test]
    fn negation_normal_form_works_with_negated_implies() {
        let result = formula_to_string(&Operator::from_formula("AB>!").unwrap());

        assert_eq!(result, "AB!|");
    }

    #[test]
    fn negation_normal_form_works_with_complicated_things() {
        let result = formula_to_string(&Operator::from_formula("AB|C&!").unwrap());

        assert_eq!(result, "A!B!&C!|");
    }
}
