// Topic: Looping using the loop statement
//
// Program requirements:
// * Display "1" through "4" in the terminal
//
// Notes:
// * Use a mutable integer variable
// * Use a loop statement
// * Print the variable within the loop statement
// * Use break to exit the loop

fn main() {
    // * Use a mutable integer variable
    let mut temp_num = 1;

    // * Use a loop statement
    loop {
        if temp_num <= 4 {
            // * Print the variable within the loop statement
            println!("{:?}", temp_num);
            temp_num += 1;
        } else {
            break;
        }
    }
}
