// Topic: User input
//
// Requirements:
// * Verify user input against pre-defined keywords
// * The keywords represent possible power options for a computer:
//   * Off
//   * Sleep
//   * Reboot
//   * Shutdown
//   * Hibernate
// * If the user enters one of the keywords, a message should be printed to
//   the console indicating which action will be taken
//   * Example: If the user types in "shutdown" a message should display such
//     as "shutting down"
// * If the keyword entered does not exist, an appropriate error message
//   should be displayed
//
// Notes:
// * Use an enum to store the possible power states
// * Use a function with a match expression to print out the power messages
//   * The function should accept the enum as an input
// * Use a match expression to convert the user input into the power state enum
// * The program should be case-insensitive (the user should be able to type
//   Reboot, reboot, REBOOT, etc.)
use std::io;
enum Options {
    Off,
    Sleep,
    Reboot,
    Shutdown,
    Hiberbate,
}

impl Options {
    fn match_power_message (&self)  {
        match self {
            Self::Off => println!("Off"),
            Self::Sleep => println!("I am going to sleep"),
            Self::Reboot => println!("Rebooting"),
            Self::Shutdown => println!("Shuting down"),
            Self::Hiberbate => println!("Going to hiberbate"),
            _ => println!()
        }
    }
}

fn main() {
    let mut user_input: String =  String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to read the line");
    user_input = user_input.to_lowercase();
    println!("{:?}", user_input);
}
