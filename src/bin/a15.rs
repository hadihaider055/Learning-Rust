// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

// * Use an enum for the tickets with data associated with each variant
enum Ticket {
    Backstage(f64, String),
    Vip(f64, String),
    Standard(f64),
}

fn main() {
    // * Create one of each ticket and place into a vector
    let tickets: Vec<Ticket> = vec![
        Ticket::Backstage(32.2, "John Doe".to_owned()),
        Ticket::Vip(52.2, "Hadi Haider".to_owned()),
        Ticket::Standard(12.2),
    ];

    // * Use a match expression while iterating the vector to print the ticket info
    for ticket in tickets {
        match ticket {
            Ticket::Backstage(price, ticketHolder) => println!(
                "{:?} owns the backstaged ticket which is of {:?}",
                ticketHolder, price
            ),
            Ticket::Vip(price, ticketHolder) => println!(
                "{:?} owns the backstaged ticket which is of {:?}",
                ticketHolder, price
            ),
            Ticket::Standard(price) => println!("Standard ticket costs {:?}", price),
        }
    }
}
