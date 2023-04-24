// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
struct Student {
    name: String,
    // * The locker assignment should use an Option<i32>
    locker: Option<i32>,
}

fn print_student_details(name: &str, locker: Option<i32>) {
    println!("Student: {:?}", name);
    match locker {
        Some(lock) => println!("Locker: {:?}", lock),
        None => print!("No locker assigned"),
    }
}

fn main() {
    let detail: Student = Student {
        name: "Hadi Haider".to_owned(),
        locker: Some(32),
    };
    print_student_details(&detail.name, detail.locker)
}
