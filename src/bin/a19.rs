// Topic: HashMap
//
// Requirements:
// * Print the name and number of items in stock for a furniture store
// * If the number of items is 0, print "out of stock" instead of 0
// * The store has:
//   * 5 Chairs
//   * 3 Beds
//   * 2 Tables
//   * 0 Couches
// * Print the total number of items in stock
//
// Notes:
// * Use a HashMap for the furniture store stock

// * Use a HashMap for the furniture store stock
use std::collections::HashMap;

#[derive(Hash, Eq, PartialEq, Debug)]
struct Furniture {
    item: String,
}

fn main() {
    let mut furniture_stock = HashMap::new();

    // * The store has:
    //   * 5 Chairs
    //   * 3 Beds
    //   * 2 Tables
    //   * 0 Couches
    furniture_stock.insert(
        Furniture {
            item: "Chairs".to_owned(),
        },
        5,
    );
    furniture_stock.insert(
        Furniture {
            item: "Beds".to_owned(),
        },
        3,
    );
    furniture_stock.insert(
        Furniture {
            item: "Tables".to_owned(),
        },
        2,
    );
    furniture_stock.insert(
        Furniture {
            item: "Couches".to_owned(),
        },
        0,
    );

    let mut total_items = 0;

    // * Print the name and number of items in stock for a furniture store
    for (furniture, stock) in furniture_stock.iter() {
        total_items += stock;
        match stock {
            // * If the number of items is 0, print "out of stock" instead of 0
            0 => println!("{}: Out of stock", furniture.item),
            _ => println!("{}: {}", furniture.item, stock),
        }
    }

    // * Print the total number of items in stock
    println!("Total items in stock: {}", total_items);
}
