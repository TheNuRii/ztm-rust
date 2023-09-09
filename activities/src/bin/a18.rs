// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
#[derive(Debug)]
struct Adult {
    name: String,
    age: u8,
}

// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created

impl Adult {
    fn is_adult(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            return Ok(Self { 
                name: name.to_string(), 
                age, 
            });
        }
        else {
            return Err("You must be at least 21");
        }
    }
}
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

fn main() {
    let adult = Adult::is_adult(22, "Jacob");
    let child: Result<Adult, &str> = Adult::is_adult(17, "Tom");

    match adult {
        Ok(adult) => println!("{:?}, {:?}", adult.name, adult.age),
        Err(err) => println!("{:?}", err),
    }

    match child {
        Ok(child) => println!("{:?}, {:?}", child.name, child.age),
        Err(err) => println!("{:?}", err),
    }
}
