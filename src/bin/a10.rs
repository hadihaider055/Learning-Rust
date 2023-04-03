// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

// * Use a function to print the messages
fn messages(value: bool) {
    // * Use a match expression to determine which message
    //   to print
    match value {
        true => println!("its big"),
        false => println!("its small"),
    }
}

fn main() {
    let value = 101;

    // * Use a boolean variable set to the result of
    //   an if..else expression to store whether the value
    //   is > 100 or <= 100
    let check_val = value > 100;

    messages(check_val)
}
