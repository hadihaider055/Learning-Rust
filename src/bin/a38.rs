// Topic: Multithreading
//
// Requirements:
// * Run the provided functions in threads
// * Retrieve the data from the threads to print the message
//   "Hello, threads!"
//
// Notes:
// * Use the join function to wait for threads to finish

use std::thread;
use std::thread::JoinHandle;

fn msg_hello() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "Hello, "
}

fn msg_thread() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "threads"
}

fn msg_excited() -> &'static str {
    use std::time::Duration;
    std::thread::sleep(Duration::from_millis(1000));
    "!"
}

fn main() {
    let msg_hello = thread::spawn(|| msg_hello());
    let msg_thread = thread::spawn(|| msg_thread());
    let msg_excited = thread::spawn(|| msg_excited());

    let msg_one = msg_hello.join().expect("failed to load msg_hello"); // we can try with both ways "With this we will return a custom message if thread failed"
    let msg_two = msg_thread.join().unwrap(); // This will return error directly without custimzation
    let msg_three = msg_excited.join().unwrap();

    println!("{}{}{}", msg_one, msg_two, msg_three);
}
