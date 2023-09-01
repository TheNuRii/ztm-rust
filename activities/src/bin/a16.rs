// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>
struct Lockers {
    student_name: String,
    locker_number: Option<i32>,
}

fn main() {
    let students: Vec<Lockers> = vec![
    Lockers {
        student_name: "Maciej".to_owned(),
        locker_number: Some(100),  
    },

    Lockers {
        student_name: "Kris".to_owned(),
        locker_number: None,
    },
    ];

    for student in students {
        match student.locker_number {
            Some(number) => println!("Name: {:?}, locker: {:?}", student.student_name, number),
            None => println!("Name: {:?}, locker: not asigned", student.student_name),
        }
    };
}
