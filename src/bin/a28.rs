// Topic: New type pattern
//
// Requirements:
// * Display the selected color of shoes, a shirt, and pants
// * Create and display at least one of each type of clothes and color
//
// Notes:
// * Create a new type for each clothing item that wraps the Color enum
//   * Each new type should implement a `new` function
// * Create a function for each type of clothes (shoes, shirt, pants)
//   that accepts the new type specific to that type of clothing

#[derive(Debug)]
enum Color {
    Black,
    Blue,
    Brown,
    Custom(String),
    Gray,
    Green,
    Purple,
    Red,
    White,
    Yellow,
}

#[derive(Debug)]
struct ShoesColor(Color);

impl ShoesColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

#[derive(Debug)]
struct ShirtColor(Color);

impl ShirtColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}
#[derive(Debug)]
struct PantsColor(Color);

impl PantsColor {
    fn new(color: Color) -> Self {
        Self(color)
    }
}

fn print_shoes_color(color: ShoesColor) {
    println!("{:?}", color);
}
fn print_shirt_color(color: ShirtColor) {
    println!("{:?}", color);
}
fn print_pant_color(color: PantsColor) {
    println!("{:?}", color);
}

fn main() {
    let new_shoes: ShoesColor = ShoesColor::new(Color::White);
    let new_shirt: ShirtColor = ShirtColor::new(Color::Blue);
    let new_pant: PantsColor = PantsColor::new(Color::Black);

    print_shoes_color(new_shoes);
    print_shirt_color(new_shirt);
    print_pant_color(new_pant);
}
