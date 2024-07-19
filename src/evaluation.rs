#[derive(PartialEq, Debug, Clone)]
struct Node {
    operator: Operator,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(operator: Operator, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Node {
        Node {
            operator,
            left,
            right,
        }
    }
}

fn parse_into_tree(formulate: &str) -> Node {
    if formulate.is_empty() {
        panic!("Invalid expression");
    }
    let mut operands = Vec::new();
    let mut head: Option<Node> = None;
    for char in formulate.chars() {
        let operator = Operator::from_char(char);
        if operator.is_operand() {
            operands.push(operator);
        } else {
            let a = operands.pop().expect("Invalid expression");
            let b = operands.pop().expect("Invalid expression");
            if head.is_none() {
                let new_node = Node::new(
                    operator,
                    Some(Box::new(Node::new(a, None, None))),
                    Some(Box::new(Node::new(b, None, None))),
                );
                head = Some(new_node);
            } else {
                let new_node = Node::new(
                    operator,
                    Some(Box::new(head.unwrap())),
                    Some(Box::new(Node::new(b, None, None))),
                );
                head = Some(new_node);
            }
        }
    }
    head.unwrap()
}

#[derive(PartialEq, Debug, Clone, Copy)]
enum Operator {
    True,
    False,
    Not,
    And,
    Or,
    Xor,
    Implies,
    Equals,
}

impl Operator {
    fn from_char(c: char) -> Operator {
        match c {
            '1' => Operator::True,
            '0' => Operator::False,
            '!' => Operator::Not,
            '&' => Operator::And,
            '|' => Operator::Or,
            '^' => Operator::Xor,
            '>' => Operator::Implies,
            '=' => Operator::Equals,
            _ => panic!("Invalid operator"),
        }
    }

    fn is_operand(&self) -> bool {
        match self {
            Operator::True => true,
            Operator::False => true,
            _ => false,
        }
    }
}

fn evaluate(a: Operator, b: Operator, operator: Operator) -> Operator {
    match operator {
        Operator::And => {
            if a == Operator::True && b == Operator::True {
                Operator::True
            } else {
                Operator::False
            }
        }
        Operator::Or => {
            if a == Operator::True || b == Operator::True {
                Operator::True
            } else {
                Operator::False
            }
        }
        Operator::Xor => {
            if a != b {
                Operator::True
            } else {
                Operator::False
            }
        }
        Operator::Implies => {
            if a == Operator::True && b == Operator::False {
                Operator::False
            } else {
                Operator::True
            }
        }
        Operator::Equals => {
            if a == b {
                Operator::True
            } else {
                Operator::False
            }
        }
        _ => panic!("Invalid operator"),
    }
}

pub fn eval_formula(formula: &str) -> bool {
    if formula.is_empty() {
        return true;
    }
    let mut operands = Vec::new();
    let mut head: Option<Node> = None;
    for char in formula.chars() {
        let operator = Operator::from_char(char);
        if operator.is_operand() {
            operands.push(operator);
        } else {
            let a = operands.pop().expect("Invalid expression");
            let b = operands.pop().expect("Invalid expression");
            let result = evaluate(a, b, operator);
            if head.is_none() {
                let new_node = Node::new(
                    operator,
                    Some(Box::new(Node::new(a, None, None))),
                    Some(Box::new(Node::new(b, None, None))),
                );
                head = Some(new_node);
            } else {
                let new_node = Node::new(
                    operator,
                    Some(Box::new(head.unwrap())),
                    Some(Box::new(Node::new(b, None, None))),
                );
                head = Some(new_node);
            }
            operands.push(result);
        }
    }
    operands.pop().expect("Invalid expression") == Operator::True
}

#[cfg(test)]
mod tests {
    use super::*;

    fn evaluate_formula(formula: &str, expected: bool) {
        let result = eval_formula(formula);
        assert_eq!(result, expected);
    }

    #[test]
    fn evaluating_basic_formulas_works() {
        evaluate_formula("10&", false);
        evaluate_formula("10|", true);
        evaluate_formula("10|1&", true);
        evaluate_formula("101|&", true);
        evaluate_formula("11>", true);
        evaluate_formula("10=", false);
        evaluate_formula("101|&", true);
        evaluate_formula("1011||=", true);
    }
}
