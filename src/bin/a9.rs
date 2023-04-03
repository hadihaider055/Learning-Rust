// Topic: Data management using tuples
//
// Requirements:
// * Print whether the y-value of a cartesian coordinate is
//   greater than 5, less than 5, or equal to 5
//
// Notes:
// * Use a function that returns a tuple
// * Destructure the return value into two variables
// * Use an if..else if..else block to determine what to print

// * Use a function that returns a tuple
fn cartesian_coordinates() -> (i32, i32) {
    (10, 20)
}

fn main() {
    // * Destructure the return value into two variables
    let (x, y) = cartesian_coordinates();

    // * Use an if..else if..else block to determine what to print
    if y > 5 {
        println!("y-value is greater than >5.")
    } else if y < 5 {
        println!("y-value is less than <5.")
    } else {
        println!("y-value is equals than =5.")
    }
}
