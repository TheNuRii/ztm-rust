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
enum Tickets {
    Backstage(f64, String),
    Standard(f64),
    Vip(f64, String), 
    
}

// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

fn main() {
    let tickets: Vec<Tickets> = vec![
        Tickets::Backstage(100.0, "Mark".to_owned()),
        Tickets::Standard(50.0),
        Tickets::Vip(75.0, "Jacob".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Tickets::Backstage(pirce, holder) => {
                println!("Backstage Ticket price: {:?}, Holder: {:?}", pirce, holder)
            },
            Tickets::Standard(price) => println!("Standard Tickent pirce: {:?}", price),
            Tickets::Vip(price, holder) => {
                println!("Vip Ticket pirce: {:?}, Holder: {:?}", price, holder)
            },
        }
    }
}
