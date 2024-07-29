use gray_code::gray_code;

mod adder;
mod conjunctive_normal_form;
mod curve;
mod evaluation;
mod gray_code;
mod inverse_curve;
mod multiplier;
mod negation_normal_form;
mod operator;
mod powerset;
mod sat;
mod set_evaluation;
mod truth_table;

fn main() {
    let something = 10;
    let something_else = 10;

    let result = adder::add(something, something_else);
    println!("The result from adding 10 and 10 is: {}", result);

    let result = multiplier::multiplier(4, 4);
    println!("The result from multiplicating 4 and 4 is: {}", result);

    println!("{}", gray_code(0));
    println!("{}", gray_code(1));
    println!("{}", gray_code(2));

    let expression = "10|";
    let result = evaluation::eval_formula(expression);
    println!(
        "The result from evaluating the expression '{}' is: {}",
        expression, result
    );

    truth_table::print_truth_table("ABC|&");

    let expression = "AB&!";
    let result = negation_normal_form::negation_normal_form(expression);
    println!(
        "The negation normal form of the expression '{}' is: {}",
        expression, result
    );

    let expression = "AB&!";
    let result = conjunctive_normal_form::conjunctive_normal_form(expression);
    println!(
        "The negation normal form of the expression '{}' is: {}",
        expression, result
    );

    let expression = "AB&!";
    let result = sat::sat(expression);
    println!("The expression '{}' is satisfiable: {}", expression, result);

    let set = vec![1, 2, 3];
    let result = powerset::powerset(set.clone());
    println!("The powerset of the set {:?} is: {:?}", set, result);

    let set = vec![vec![1], vec![1, 2]];
    let formula = "AB&";
    let result = set_evaluation::eval_set(formula, set.clone());
    println!(
        "The result from evaluating the formula '{}' with the set {:?} is: {:?}",
        formula, set, result
    );

    let x = 10;
    let y = 10;
    let result = curve::map(x, y);
    println!(
        "The result from mapping the values {} and {} is: {}",
        x, y, result
    );

    let inverse = inverse_curve::reverse_map(result);
    println!(
        "The result from reversing the mapping of {} is: {:?}",
        result, inverse
    );
}
