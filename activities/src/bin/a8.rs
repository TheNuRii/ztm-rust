// Topic: Organizing similar data using structs
//
// Requirements:
// * Print the flavor of a drink and it's fluid ounces
//
// Notes:
// * Use an enum to create different flavors of drinks
// * Use a struct to store drink flavor and fluid ounce information
// * Use a function to print out the drink flavor and ounces
// * Use a match expression to print the drink flavor
enum Flavor {
    OrangeJuce,
    AppleJuce,
    Beer,
    Water,
}

struct Drink {
    drink_flavor: Flavor,
    capacity_liters: f64,
}

fn print_drink (drink: Drink){
    match drink.drink_flavor {
        Flavor::AppleJuce => println!("flavor: AppleJuce"),
        Flavor::Beer => println!("flavor: Beer"),
        Flavor::Water => println!("flavor: Water"),
        Flavor::OrangeJuce => println!("flavor: OrangeJuce"),
    }
    println!("liters: {:?}", drink.capacity_liters);
}
fn main() {
    let favorite_drink = Drink {
        drink_flavor: Flavor::Beer,
        capacity_liters: 0.5,
    };
    print_drink(favorite_drink);
}

