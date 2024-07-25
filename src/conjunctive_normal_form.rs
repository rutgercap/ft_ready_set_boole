use crate::operator::Operator;

fn conjunctive_normal_form(formula: &str) -> String {
    unimplemented!()
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
    fn conjunctive_normal_form_works_with_and() {
        let result = conjunctive_normal_form("AB&!");

        assert_eq!(result, "A!B!|");
    }

    #[test]
    fn conjunctive_normal_form_works_with_or() {
        let result = conjunctive_normal_form("AB|");

        assert_eq!(result, "AB|");
    }
}