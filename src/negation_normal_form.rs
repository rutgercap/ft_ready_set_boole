
pub fn negation_normal_form(formula: &str) -> String {
    "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negation_normal_form_works_with_empty_string() {
        let result = negation_normal_form("");

        assert_eq!(result, "");
    }
}