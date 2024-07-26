use crate::operator::Operator;

impl Operator {
    fn to_conjunctive_normal_form(self) -> Operator {
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
    operands
        .unwrap()
        .to_negation_normal_form()
        .to_conjunctive_normal_form()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conjunctive_normal_form_works_with_empty_string() {
        let result = conjunctive_normal_form("");

        assert_eq!(result, "");
    }

    #[test]
    fn conjunctive_normal_form_works_with_negated_and() {
        let result = conjunctive_normal_form("AB&!");

        assert_eq!(result, "A!B!|");
    }

    #[test]
    fn conjunctive_normal_form_works_with_negated_or() {
        let result = conjunctive_normal_form("AB|!");

        assert_eq!(result, "A!B!&");
    }

    #[test]
    fn conjunctive_normal_form_works_with_deep_and() {
        let result = conjunctive_normal_form("AB|C&");

        assert_eq!(result, "AB|C&");
    }

    #[test]
    fn conjunctive_normal_form_works_with_1() {
        let result = conjunctive_normal_form("ABCD&|&");

        assert_eq!(result, "ABC|BD|&&");
    }

    #[test]
    fn conjunctive_normal_form_works_with_many_ors() {
        let result = conjunctive_normal_form("AB|C|D|");

        assert_eq!(result, "AB|C|D|");
    }

    #[test]
    fn conjunctive_normal_form_works_with_many_ands() {
        let result = conjunctive_normal_form("AB&C&D&");

        assert_eq!(result, "AB&C&D&");
    }

    #[test]
    fn conjunctive_normal_form_works_complicated() {
        let result = conjunctive_normal_form("AB&!C!|");

        assert_eq!(result, "A!B!|C!|");
    }
}
