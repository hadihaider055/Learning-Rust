// Topic: Vectors
//
// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector
//
// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for..in loop
// * Determine whether to print the number or print "thirty" inside the loop
// * Use the .len() function to print the number of elements in a vector

fn main() {
    // * Use a vector to store 4 numbers
    let my_numbers = vec![10, 20, 30, 40];

    // * Iterate through the vector using a for..in loop
    for my_num in &my_numbers {
        match my_num {
            // * Determine whether to print the number or print "thirty" inside the loop
            30 => println!("The number is = thirty"),
            _ => println!("The number is = {:?}", my_num),
        }
    }

    // * Use the .len() function to print the number of elements in a vector
    println!("The length = {:?}", my_numbers.len());
}
