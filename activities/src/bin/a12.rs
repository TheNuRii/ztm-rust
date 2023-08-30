// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
enum Color {
    Red,
    Blue,
    Green,
}

impl Color {
    fn print(&self){
        match self {
            Color::Red=> println!("color: Red"),
            Color::Blue=> println!("color: Blue"),
            Color::Green => println!("color: Green"),
        }
    }
}

struct ShippingBox {
    dimensions: Dimensions,
    weight: f64,
    color: Color,
}
// * Use an enum for the box color
struct Dimensions {
    width: f64,
    height: f64,
    depth: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("width: {:?}", self.width);
        println!("height: {:?}", self.height);
        println!("depth: {:?}", self.depth);
    }
}

//* Implement functionality on the box struct to print the characteristics

impl ShippingBox {
    // * Implement functionality on the box struct to create a new box
    fn create_box(dimensions: Dimensions, weight: f64, color: Color ) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn print(&self) {
        self.color.print();
        self.dimensions.print();
        println!("weight: {:?}", self.weight);
    }
}

fn main() {
    let create_dimenstions = Dimensions { 
        width: 100.0, 
        height: 10.0, 
        depth: 10.0, 
    };

    let create_box = ShippingBox::create_box(create_dimenstions, 200.0, Color::Blue);

    create_box.print();
}
