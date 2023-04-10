// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

// * Use a struct for the grocery item
struct GroceryItem {
    // * Use two i32 fields for the quantity and id number
    id: i32,
    quantity: i32,
}

// * Create a function to display the quantity, with the struct as a parameter
fn display_quantity(grocery: &GroceryItem) {
    println!("Quantity = {:?}", grocery.quantity)
}

// * Create a function to display the id number, with the struct as a parameter
fn display_id(grocery: &GroceryItem) {
    println!("ID = {:?}", grocery.id)
}

fn main() {
    let grocery_item = GroceryItem {
        id: 32,
        quantity: 32,
    };

    display_id(&grocery_item);
    display_quantity(&grocery_item);
}
