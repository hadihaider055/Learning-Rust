// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor

// * Use an enum to create different flavors of drinks
enum Flavors {
    Apricot,
    BirchBeer,
    Citron,
    Chocolate,
}

// * Use a struct to store drink flavor and fluid ounce information
struct FlavorOunce {
    flavor: Flavors,
    ounce: f64,
}

// * Use a function to print out the drink flavor and ounces
fn print_drink(drink: FlavorOunce) {
    // * Use a match expression to print the drink flavor
    match drink.flavor {
        Flavors::Apricot => println!("Apricot"),
        Flavors::BirchBeer => println!("BirchBeer"),
        Flavors::Citron => println!("Citron"),
        Flavors::Chocolate => println!("Chocolate"),
    }

    println!("The oz: {:?}", drink.ounce)
}

fn main() {
    let d = FlavorOunce {
        flavor: Flavors::Citron,
        ounce: 4.9,
    };

    print_drink(d)
}
