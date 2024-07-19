#[derive(PartialEq, Debug, Clone, Copy)]
enum Node {
    Operator(Operator),
    Operand(char),
}

impl Node {
    fn to_operand(self) -> char {
        match self {
            Node::Operand(c) => c,
            _ => panic!("Node is not an operand"),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Operator {
    And(char, char),
    Or(char, char),
    Xor(char, char),
    Implies(char, char),
    Equals(char, char),
    Not(char),
}

impl Operator {
    fn from_char(c: char, a: char, b: char) -> Operator {
        let a = a.to_uppercase().next().unwrap();
        let b = b.to_uppercase().next().unwrap();
        match c {
            '&' => Operator::And(a, b),
            '|' => Operator::Or(a, b),
            '^' => Operator::Xor(a, b),
            '>' => Operator::Implies(a, b),
            '=' => Operator::Equals(a, b),
            _ => panic!("Invalid operator"),
        }
    }

    fn not(a: char) -> Operator {
        Operator::Not(a.to_uppercase().next().unwrap())
    }
}

fn nodes_from_formula(formula: &str) -> Vec<Node> {
    let mut stack: Vec<Node> = Vec::new();

    for token in formula.chars() {
        if token.is_alphabetic() {
            stack.push(Node::Operand(token));
            continue;
        }
        match token {
            '!' => {
                let operand = stack.pop().unwrap().to_operand();
                let node = Node::Operator(Operator::not(operand));
                stack.push(node);
            },
            _ => {
                let right = stack.pop().unwrap().to_operand();
                let left = stack.pop().unwrap().to_operand();
                let node = Node::Operator(Operator::from_char(token, left, right));
                stack.push(node);
            }
        }
    }
    stack
}

pub fn print_truth_table(formula: &str) {
    let stack = nodes_from_formula(formula);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluating_basic_formulas_works() {
        let nodes = nodes_from_formula("AB&");

        assert_eq!(nodes, vec![Node::Operator(Operator::And('A', 'B'))]);
    }

    #[test]
    fn evaluating_not_works() {
        let nodes = nodes_from_formula("A!");

        assert_eq!(nodes, vec![Node::Operator(Operator::Not('A'))]);
    }

    #[test]
    fn evaluating_empty_string_works() {
        let nodes = nodes_from_formula("");

        assert_eq!(nodes, vec![]);
    }

    #[test]
    fn evaluationg_formula_with_multiple_operators_works() {
        let tree = nodes_from_formula("AB&C|");

        assert_eq!(tree, vec![
            Node::Operator(Operator::Or(Operator::And('A', 'B'), 'C')),
        ]);
    }

    #[test]
    fn evalauting_complicated_formula_works() {
        let tree = nodes_from_formula("ABC|&");

    }

    #[test]
    fn evaluate_another_formula() {
        let tree = nodes_from_formula("AB!CD&|");
    }
}
