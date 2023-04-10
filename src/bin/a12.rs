// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

// * Use an enum for the box color
enum BoxColor {
    Brown,
    Yellow,
}

impl BoxColor {
    fn print(&self) {
        match self {
            BoxColor::Brown => println!("Color = Brown"),
            BoxColor::Yellow => println!("Color = Yellow"),
        }
    }
}

struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Width = {:?}", self.width);
        println!("Height = {:?}", self.height);
        println!("Depth = {:?}", self.depth);
    }
}
// * Use a struct to encapsulate the box characteristics
struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: BoxColor,
}

impl ShippingBox {
    // * Implement functionality on the box struct to create a new box
    fn create_box(weight: f64, dimensions: Dimensions, color: BoxColor) -> Self {
        Self {
            weight,
            dimensions,
            color,
        }
    }

    // * Implement functionality on the box struct to print the characteristics
    fn display_char(&self) {
        self.dimensions.print();
        self.color.print();
        println!("Weight = {:?}", self.weight);
    }
}

fn main() {
    let box_dimensions = Dimensions {
        width: 3.0,
        height: 2.0,
        depth: 1.0,
    };

    let shipping_box = ShippingBox::create_box(3.1, box_dimensions, BoxColor::Brown);
    shipping_box.display_char();
}
