// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
struct Person {
    name: String,
    favorite_color: String,
    age: i32,
}
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function
fn print_name(name: &str) {
    println!("Name: {:?}", name);
}

fn print_color(color: &str) {
    println!("Color: {:?}", color);
}

fn main() {
    let people = vec![
        Person {
            name: "Maciej".to_owned(),
            favorite_color: "blue".to_owned(),
            age: 20,
        },

        Person {
            name: "Kuba".to_owned(),
            favorite_color: "red".to_owned(),
            age: 8,
        },

        Person {
            name: "Wojetek".to_owned(),
            favorite_color: "green".to_owned(),
            age: 10,
        },
    ];
    
    for person in people {
        if person.age <= 10 {
            print_color(&person.favorite_color);
            print_name(&person.name);
        }
    } 
}
