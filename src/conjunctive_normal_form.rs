use crate::operator::Operator;

fn conjunctive_normal_form(formula: &str) -> String {
    return "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conjunctive_normal_form_works_with_empty_string() {
        let result = conjunctive_normal_form("");

        assert_eq!(result, "");
    }

}