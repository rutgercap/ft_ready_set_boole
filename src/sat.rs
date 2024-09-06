use std::collections::HashSet;

use crate::operator::Operator;

pub fn sat(formula: &str) -> bool {
    let operator = Operator::from_formula(formula);
    if operator.is_none() {
        return true;
    }
    let operator = operator.unwrap();
    let operator = operator
        .to_negation_normal_form()
        .to_conjunctive_normal_form();
    let mut clauses = operator.extract_clauses();
    let mut assignments = HashSet::new();
    Operator::dpll(&mut clauses, &mut assignments)
}

impl Operator {
    pub fn extract_clauses(&self) -> Vec<Vec<Operator>> {
        match self {
            Operator::And(lhs, rhs) => {
                let mut left_clauses = lhs.extract_clauses();
                let mut right_clauses = rhs.extract_clauses();
                left_clauses.append(&mut right_clauses);
                left_clauses
            }
            _ => vec![self.extract_literals()],
        }
    }

    pub fn extract_literals(&self) -> Vec<Operator> {
        match self {
            Operator::Or(lhs, rhs) => {
                let mut left_literals = lhs.extract_literals();
                let mut right_literals = rhs.extract_literals();
                left_literals.append(&mut right_literals);
                left_literals
            }
            _ => vec![self.clone()],
        }
    }

    pub fn negate(&self) -> Operator {
        match self {
            Operator::Not(inner) => *inner.clone(),
            _ => Operator::not(self.clone()),
        }
    }

    fn dpll(clauses: &mut Vec<Vec<Operator>>, assignments: &mut HashSet<Operator>) -> bool {
        if clauses.is_empty() {
            return true;
        }
        if clauses.iter().any(|clause| clause.is_empty()) {
            return false;
        }

        for clause in clauses.clone().iter() {
            if clause.len() == 1 {
                let literal = &clause[0];
                let negated = literal.negate();
                assignments.insert(literal.clone());
                clauses.retain(|c| !c.contains(literal)); 
                for c in clauses.iter_mut() {
                    c.retain(|l| *l != negated); 
                }
                return Operator::dpll(clauses, assignments);
            }
        }

        let literal = clauses[0][0].clone(); 
        let negated = literal.negate();

        let mut new_clauses_true = clauses.clone();
        new_clauses_true.retain(|c| !c.contains(&literal)); 
        for c in new_clauses_true.iter_mut() {
            c.retain(|l| *l != negated); 
        }
        if Operator::dpll(&mut new_clauses_true, assignments) {
            return true;
        }

        // Try assigning false to the literal
        let mut new_clauses_false = clauses.clone();
        new_clauses_false.retain(|c| !c.contains(&negated)); 
        for c in new_clauses_false.iter_mut() {
            c.retain(|l| *l != literal);
        }
        if Operator::dpll(&mut new_clauses_false, assignments) {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_clause_is_satisfiable() {
        assert_eq!(sat(""), true);
    }

    #[test]
    fn base_case() {
        assert_eq!(sat("A"), true);
    }

    #[test]
    fn test_sat() {
        assert_eq!(sat("AB|"), true);
    }

    #[test]
    fn sat_works_with_other_operators() {
        assert_eq!(sat("AA^"), false);
    }

    #[test]
    fn single_literal_and_negation() {
        assert_eq!(sat("AA!&"), false);
    }

    #[test]
    fn disjunction_of_literals() {
        assert_eq!(sat("AB|"), true);
    }

    #[test]
    fn conjunction_of_literals_and_negations() {
        assert_eq!(sat("AB!&"), true);
    }

    #[test]
    fn complex_cnf_expression() {
        assert_eq!(sat("AB|AC|&"), true);
    }
}
