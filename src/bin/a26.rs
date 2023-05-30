// Topic: External crates
//
// Requirements:
// * Display the current date and time
//
// Notes:
// * Use the `chrono` crate to work with time
// * (OPTIONAL) Read the documentation section `Formatting and Parsing`
//   for examples on how to create custom time formats

use chrono::prelude::*;

fn main() {
    // * Display the current date and time
    let local_time: DateTime<Local> = Local::now();
    let parse_local_time = local_time.format("%Y-%m-%d %H:%M:%S").to_string();
    println!("Current date and time is {:?}", parse_local_time);
}
