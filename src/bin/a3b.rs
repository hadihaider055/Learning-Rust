// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
    // * Use a variable set to any integer value
    let num = 6;
    let match_num = 5;

    // * Use an if..else if..else block to determine which message to display
    if num > match_num {
        // * Use the println macro to display messages to the terminal
        println!("The number is greater than (>){}", match_num);
    } else if num < match_num {
        // * Use the println macro to display messages to the terminal
        println!("The number is less than (<){}", match_num);
    } else {
        // * Use the println macro to display messages to the terminal
        println!("The number is equals to than (=){}", match_num);
    }
}
