mod adder;
mod multiplier;

fn main() {
    let something = 10;
    let something_else = 10;

    let result = adder::add(something, something_else);
    println!("The result from adding 10 and 10 is: {}", result);

    let result = multiplier::multiplier(4, 4);
    println!("The result from multiplicating 4 and 4 is: {}", result);
}
