use crate::operator::Operator;

impl Operator {


    fn to_conjunctive_normal_form(self) -> Operator {
        fn distribute_or(a: &Operator, b: &Operator) -> Operator {
            match (a, b) {
                (Operator::And(c, d), e) => {
                    Operator::and(
                        distribute_or(c, e),
                        distribute_or(d, e),
                    )
                },
                (e, Operator::And(c, d)) => {
                    Operator::and(
                        distribute_or(e, c),
                        distribute_or(e, d),
                    )
                },
                _ => Operator::or(a.clone(), b.clone()),
            }
        }
        match self {
            Operator::Operand(_) => self,
            Operator::Not(a) => Operator::not(a.to_conjunctive_normal_form()),
            Operator::Or(a, b) => {
                let a = a.to_conjunctive_normal_form();
                let b = b.to_conjunctive_normal_form();
                distribute_or(&a, &b)
            }
            Operator::And(a, b) => Operator::and(
                a.to_conjunctive_normal_form(),
                b.to_conjunctive_normal_form(),
            ),
            _ => panic!("This operator should not occur if the formula is in NNF"),
        }
    }

    fn to_conjunctive_normal_form_string(self) -> String {
        let mut result = "";
        match self {
            Operator::And(a, b) => format!("{}{}", a.to_conjunctive_normal_form_string(), b.to_conjunctive_normal_form_string()),
            Operator::Or(a, b) => format!("{}{}|", a.to_conjunctive_normal_form_string(), b.to_conjunctive_normal_form_string()),
            Operator::Not(c) => format!("{}!", c.to_conjunctive_normal_form_string()),
            Operator::Operand(c) => c.to_string(),
            _ => panic!("This operator should not occur if the formula is in NNF"),
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
    fn conjunctive_normal_form_works_with_many_ors() {
        let result = conjunctive_normal_form("AB|C|D|");

        assert_eq!(result, "ABCD|||");
    }

        // Or(Operand('A'), Or(Operand('B'), Or(Operand('C'), Operand('D'))))
        // Or(Or(Or(Operand('A'), Operand('B')), Operand('C')), Operand('D'))
    #[test]
    fn test() {
        let result = Operator::from_formula("AB|C|D|").unwrap();

        let cnf = result.clone().to_conjunctive_normal_form();
        let cnf_string = cnf;
        println!("{:?}", cnf_string);
        assert_eq!(cnf_string.to_string(), "ABCD|||");
    }
}
