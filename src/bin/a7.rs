// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

// * Use an enum with color names as variants
enum Colors {
    Red,
    Green,
    Blue,
}

// * Use a function to print the color name
// * The function must use the enum as a parameter
fn color_name(_color: Colors) {
    // * Use a match expression to determine which color
    match _color {
        Colors::Red => println!("Red Color"),
        Colors::Green => println!("Green Color"),
        Colors::Blue => println!("Blue Color"),
    }
}
fn main() {
    //   name to print
    color_name(Colors::Red)
}
