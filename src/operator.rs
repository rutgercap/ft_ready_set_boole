#[derive(PartialEq, Debug, Clone, Hash, Eq)]
pub enum Operator {
    And(Box<Operator>, Box<Operator>),
    Or(Box<Operator>, Box<Operator>),
    Xor(Box<Operator>, Box<Operator>),
    Implies(Box<Operator>, Box<Operator>),
    Equals(Box<Operator>, Box<Operator>),
    Not(Box<Operator>),
    Operand(char),
}

impl Operator {
    pub fn operand(a: char) -> Operator {
        Operator::Operand(a.to_uppercase().next().unwrap())
    }

    pub fn with_two(new: char, a: Operator, b: Operator) -> Operator {
        match new {
            '&' => Operator::And(Box::new(a), Box::new(b)),
            '|' => Operator::Or(Box::new(a), Box::new(b)),
            '^' => Operator::Xor(Box::new(a), Box::new(b)),
            '>' => Operator::Implies(Box::new(a), Box::new(b)),
            '=' => Operator::Equals(Box::new(a), Box::new(b)),
            _ => panic!("Invalid operator"),
        }
    }

    pub fn and(a: Operator, b: Operator) -> Operator {
        Operator::with_two('&', a, b)
    }

    pub fn or(a: Operator, b: Operator) -> Operator {
        Operator::with_two('|', a, b)
    }

    pub fn not(a: Operator) -> Operator {
        Operator::Not(Box::new(a))
    }

    pub fn from_formula(formula: &str) -> Option<Operator> {
        let mut stack: Vec<Operator> = Vec::new();

        for token in formula.chars() {
            if token.is_alphabetic() {
                stack.push(Operator::operand(token));
                continue;
            }
            match token {
                '!' => {
                    let operand = stack.pop().expect("No operand to negate");
                    let node = Operator::not(operand);
                    stack.push(node);
                }
                _ => {
                    let right = stack.pop().expect("Not enough operators on stack");
                    let left = stack.pop().expect("Not enough operators on stack");
                    let node = Operator::with_two(token, left, right);
                    stack.push(node);
                }
            }
        }
        if stack.len() > 1 {
            panic!("Too many operands");
        }
        stack.pop()
    }

    pub fn to_string(&self) -> String {
        match self {
            Operator::And(a, b) => format!("{}{}&", a.to_string(), b.to_string()),
            Operator::Or(a, b) => format!("{}{}|", a.to_string(), b.to_string()),
            Operator::Xor(a, b) => format!("{}{}^", a.to_string(), b.to_string()),
            Operator::Implies(a, b) => format!("{}{}>", a.to_string(), b.to_string()),
            Operator::Equals(a, b) => format!("{}{}=", a.to_string(), b.to_string()),
            Operator::Not(c) => format!("{}!", c.to_string()),
            Operator::Operand(c) => c.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluating_basic_formulas_works() {
        let nodes = Operator::from_formula("AB&");

        assert_eq!(
            nodes.unwrap(),
            Operator::with_two('&', Operator::operand('A'), Operator::operand('B'))
        );
    }

    #[test]
    fn evaluating_not_works() {
        let nodes = Operator::from_formula("A!");

        assert_eq!(nodes.unwrap(), Operator::not(Operator::Operand('A')));
    }

    #[test]
    fn evaluating_double_not_works() {
        let nodes = Operator::from_formula("A!!");

        assert_eq!(
            nodes.unwrap(),
            Operator::not(Operator::not(Operator::Operand('A')))
        );
    }

    #[test]
    fn evaluating_empty_string_works() {
        let nodes = Operator::from_formula("");

        assert_eq!(nodes, None);
    }

    #[test]
    fn evaluating_simple_formula_works() {
        let nodes = Operator::from_formula("A").unwrap();

        assert_eq!(nodes, Operator::operand('A'));
    }

    #[test]
    #[should_panic]
    fn evaluating_string_with_too_many_operands_throws_error() {
        let _ = Operator::from_formula("ABB&");
    }

    #[test]
    fn evaluationg_formula_with_multiple_operators_works() {
        let tree = Operator::from_formula("AB&C|").unwrap();

        assert_eq!(
            tree,
            Operator::with_two(
                '|',
                Operator::with_two('&', Operator::operand('A'), Operator::operand('B')),
                Operator::operand('C')
            )
        );
    }

    #[test]
    fn evalauting_complicated_formula_works() {
        let tree = Operator::from_formula("ABC|&").unwrap();

        assert_eq!(
            tree,
            Operator::with_two(
                '&',
                Operator::operand('A'),
                Operator::with_two('|', Operator::operand('B'), Operator::operand('C')),
            )
        );
    }

    #[test]
    fn evaluate_another_formula() {
        let tree = Operator::from_formula("ABCD||=").unwrap();

        assert_eq!(
            tree,
            Operator::with_two(
                '=',
                Operator::Operand('A'),
                Operator::with_two(
                    '|',
                    Operator::operand('B'),
                    Operator::with_two('|', Operator::operand('C'), Operator::operand('D'),),
                ),
            )
        );
    }
}
