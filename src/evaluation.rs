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
    for char in formula.chars() {
        let operator = Operator::from_char(char);
        if operator.is_operand() {
            operands.push(operator);
        } else {
            let a = operands.pop().expect("Invalid expression");
            let b = operands.pop().expect("Invalid expression");
            let result = evaluate(a, b, operator);
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

    #[test]
    fn evaluating_empty_formulas_works() {
        evaluate_formula("", true);
    }
}
