// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn match_guards(tile: Tile) {
    use Tile::*;

    match tile {
        Brick(brick @ BrickStyle::Gray | brick @ BrickStyle::Red) => {
            println!("The brick color is {brick:?}")
        }
        Brick(other) => println!("{other:?} brick"),
        Water(pressure) if pressure.0 >= 10 => println!("High water pressure!"),
        Water(pressure) if pressure.0 < 10 => println!("Water pressure level: {pressure:?}"),
        Grass | Dirt | Sand => println!("Ground tile"),
        Treasure(TreasureChest {
            content: TreasureItem::Gold,
            amount,
        }) if amount >= 100 => {
            println!("Lots of gold!");
        }
        _ => (),
    }
}

fn main() {
    // let tile = Tile::Brick(BrickStyle::Dungeon);

    // let tile = Tile::Dirt;
    // let tile = Tile::Treasure(TreasureChest {
    //     content: TreasureItem::Gold,
    //     amount: 1312,
    // });
    let tile = Tile::Water(Pressure(4));

    match_guards(tile);
}
