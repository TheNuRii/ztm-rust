// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
struct GroceryItem {
    id: i32,
    quantity: i32,
}
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter
fn dispaly_quantity(item: &GroceryItem) {
    println!("quantity: {:?}", item.quantity);
}
fn dispaly_id(item: &GroceryItem) {
    println!("id: {:?}", item.id);
}

fn main() {
    let carrot = GroceryItem {
        id: 1,
        quantity: 10,
    };
    dispaly_quantity(&carrot);
    dispaly_id(&carrot);
}
