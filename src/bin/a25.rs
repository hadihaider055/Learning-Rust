// Topic: Traits
//
// Requirements:
// * Calculate the perimeter of a square and triangle:
//   * The perimeter of a square is the length of any side*4.
//   * The perimeter of a triangle is a+b+c where each variable
//     represents the length of a side.
// * Print out the perimeter of the shapes
//
// Notes:
// * Use a trait to declare a perimeter calculation function
// * Use a single function to print out the perimeter of the shapes
//   * The function must utilize impl trait as a function parameter

trait PerimeterCalculation {
    fn calc_perimeter(&self) -> i32;
}

// * Calculate the perimeter of a square and triangle:
struct Square {
    side: i32,
}

impl PerimeterCalculation for Square {
    //   * The perimeter of a square is the length of any side*4.
    fn calc_perimeter(&self) -> i32 {
        self.side * 4
    }
}

struct Triangle {
    side_1: i32,
    side_2: i32,
    side_3: i32,
}

impl PerimeterCalculation for Triangle {
    //   * The perimeter of a triangle is a+b+c where each variable
    //     represents the length of a side.
    fn calc_perimeter(&self) -> i32 {
        self.side_1 + self.side_2 + self.side_3
    }
}

// * Print out the perimeter of the shapes
fn print_perimeter(shape: impl PerimeterCalculation) {
    let perimeter = shape.calc_perimeter();
    println!("Perimeter = {:?}", perimeter);
}

fn main() {
    print_perimeter(Square { side: 5 });
    print_perimeter(Triangle {
        side_1: 5,
        side_2: 3,
        side_3: 7,
    })
}
