// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)

use std::io;

// * Use an enum to store the possible power states
enum PowerStates {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hibernate,
}

// * Use a match expression to convert the user input into the power state enum
impl PowerStates {
    fn power_state(state: &str) -> Option<PowerStates> {
        let state = state.trim().to_lowercase();
        match state.as_str() {
            "off" => Some(PowerStates::Off),
            "sleep" => Some(PowerStates::Sleep),
            "reboot" => Some(PowerStates::Reboot),
            "shutdown" => Some(PowerStates::Shutdown),
            "hibernate" => Some(PowerStates::Hibernate),
            _ => None,
        }
    }
}
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
fn print_power_msg(state: PowerStates) {
    use PowerStates::*;
    match state {
        Off => println!("Turning Off!"),
        Sleep => println!("Sleeping!"),
        Reboot => println!("Rebooting!"),
        Shutdown => println!("Shutting Down!"),
        Hibernate => println!("Hibernating!"),
    }
}

fn main() {
    // * The program should be case-insensitive (the user should be able to type
    //   Reboot, reboot, REBOOT, etc.)
    let mut buffer = String::new();

    println!("Enter the power state:");
    let user_input = io::stdin().read_line(&mut buffer);

    if user_input.is_ok() {
        match PowerStates::power_state(&buffer) {
            Some(state) => print_power_msg(state),
            None => println!("Invalid input"),
        }
    } else {
        println!("Something went wrong!")
    }
}
