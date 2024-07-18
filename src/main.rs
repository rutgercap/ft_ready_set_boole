use gray_code::gray_code;

mod adder;
mod multiplier;
mod gray_code;

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
}
