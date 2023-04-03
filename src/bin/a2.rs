// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to add two numbers together
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

fn main() {
    display_result()
}

// * Use a function to add two numbers together
fn add_nums(a: i32, b: i32) -> i32 {
    return a + b;
}

// * Use a function to display the result
fn display_result() {
    let result = add_nums(10, 5);

    // * Use the "{:?}" token in the println macro to display the result
    println!("{:?}", result)
}
