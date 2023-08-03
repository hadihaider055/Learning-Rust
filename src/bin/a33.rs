// Topic: Lifetimes & Functions
//
// Summary:
// Create a program that compares which string is longer (highest length).
//
// Requirements:
// * The comparison must be done using a function named `longest`
// * No data may be copied (cannot use .to_owned() or .to_string())
// * If both strings are the same length, the first one should be returned

fn longest<'a>(short: &'a str, long: &'a str) -> &'a str {
    if short.len() > long.len() {
        short
    } else {
        long
    }
}

fn main() {
    let short = "hello";
    let long = "this is a long message";
    println!("{}", longest(short, long));
}
