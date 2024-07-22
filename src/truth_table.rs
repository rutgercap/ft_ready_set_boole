#[derive(PartialEq, Debug, Clone)]
enum Operator {
    And(Box<Operator>, Box<Operator>),
    Or(Box<Operator>, Box<Operator>),
    Xor(Box<Operator>, Box<Operator>),
    Implies(Box<Operator>, Box<Operator>),
    Equals(Box<Operator>, Box<Operator>),
    Not(Box<Operator>),
    Operand(char),
}

impl Operator {
    fn operand(a: char) -> Operator {
        Operator::Operand(a.to_uppercase().next().unwrap())
    }

    fn with_two(new: char, a: Operator, b: Operator) -> Operator {
        match new {
            '&' => Operator::And(Box::new(a), Box::new(b)),
            '|' => Operator::Or(Box::new(a), Box::new(b)),
            '^' => Operator::Xor(Box::new(a), Box::new(b)),
            '>' => Operator::Implies(Box::new(a), Box::new(b)),
            '=' => Operator::Equals(Box::new(a), Box::new(b)),
            _ => panic!("Invalid operator"),
        }
    }

    fn not(a: Operator) -> Operator {
        Operator::Not(Box::new(a))
    }
}

fn nodes_from_formula(formula: &str) -> Option<Operator> {
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
                println!("Stack: {:?}", stack);
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

fn operands_in_formula(formula: &str) -> Vec<char> {
    formula
        .chars()
        .filter(|c| c.is_alphabetic())
        .fold(vec![], |mut acc, c| {
            if !acc.contains(&c) {
                acc.push(c);
            }
            acc
        })
}

fn print_header(operands: &[char]) {
    let mut temp: String = operands.iter().map(|c| format!("| {} ", c)).collect();
    temp.push_str("| = |");
    println!("{}", temp);
}


pub fn print_truth_table(formula: &str) {
    let stack_option = nodes_from_formula(formula);
    if stack_option.is_none() {
        return;
    }
    let stack = stack_option.unwrap();

    let operands = operands_in_formula(formula);
    print_header(&operands);

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluating_basic_formulas_works() {
        let nodes = nodes_from_formula("AB&");

        assert_eq!(
            nodes.unwrap(),
            Operator::with_two('&', Operator::operand('A'), Operator::operand('B'))
        );
    }

    #[test]
    fn getting_operands_from_formula_works() {
        let operands = operands_in_formula("AB&");

        assert_eq!(operands, vec!['A', 'B',]);
    }

    #[test]
    fn getting_operands_from_formula_does_not_count_doubles() {
        let operands = operands_in_formula("ABB&");

        assert_eq!(operands, vec!['A', 'B',]);
    }

    #[test]
    fn getting_operands_from_formula_works_for_complicated_formulas() {
        let operands = operands_in_formula("ABCD||=E");

        assert_eq!(operands, vec!['A', 'B', 'C', 'D', 'E',]);
    }

    #[test]
    fn evaluating_not_works() {
        let nodes = nodes_from_formula("A!");

        assert_eq!(nodes.unwrap(), Operator::not(Operator::Operand('A')));
    }

    #[test]
    fn evaluating_empty_string_works() {
        let nodes = nodes_from_formula("");

        assert_eq!(nodes, None);
    }

    #[test]
    fn evaluating_simple_formula_works() {
        let nodes = nodes_from_formula("A").unwrap();

        assert_eq!(nodes, Operator::operand('A'));
    }

    #[test]
    #[should_panic]
    fn evaluating_string_with_too_many_operands_throws_error() {
        let _ = nodes_from_formula("ABB&");
    }

    #[test]
    fn evaluationg_formula_with_multiple_operators_works() {
        let tree = nodes_from_formula("AB&C|").unwrap();

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
        let tree = nodes_from_formula("ABC|&").unwrap();

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
        let tree = nodes_from_formula("ABCD||=").unwrap();

        assert_eq!(
            tree,
            Operator::with_two(
                '=',
                Operator::with_two(
                    '|',
                    Operator::with_two('|', Operator::operand('1'), Operator::operand('0'),),
                    Operator::operand('1'),
                ),
                Operator::operand('1'),
            )
        );
    }
}
