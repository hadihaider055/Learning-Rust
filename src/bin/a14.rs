// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// * Use a struct for a persons age, name, and favorite color
struct Person {
    age: i32,
    // * The color and name should be stored as a String
    name: String,
    fav_color: String,
}

// * The name and colors should be printed using a function
fn print_data(name: &str, color: &str) {
    println!("name: {:?}, favorite color: {:?}", name, color)
}

fn main() {
    // * Create and store at least 3 people in a vector
    let persons = vec![
        Person {
            age: 21,
            name: "Hadi".to_owned(),
            fav_color: String::from("Blue"),
        },
        Person {
            age: 22,
            name: "Yashua".to_owned(),
            fav_color: String::from("Red"),
        },
        Person {
            age: 20,
            name: "Haseeb".to_owned(),
            fav_color: String::from("Yello"),
        },
    ];

    // * Iterate through the vector using a for..in loop
    for person in persons {
        // * Use an if expression to determine which person's info should be printed
        if person.age >= 21 {
            print_data(&person.name, &person.fav_color)
        }
    }
}
