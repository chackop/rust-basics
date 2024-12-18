// location
enum Location {
    Unknown,
    Anonymous,
    Known(f64, f64), // latitude, longitude
}

impl Location {
    fn display(&self) {
        match *self {
            Location::Unknown => println!("Unknown Location"),
            Location::Anonymous => println!("Anonymous Location"),
            Location::Known(lat, lon) => println!("{}, {}", lat, lon),
        }
    }
}

fn main() {
    let address = Location::Unknown;
    address.display();
    let address = Location::Anonymous;
    address.display();
    let address = Location::Known(28.608295, -80.604177);
    address.display();
}

// // if let syntax
// fn main() {
//     let number = None;

//     if let Some(13) = number {
//         println!("thirteen");
//     }
// }

// // match option
// fn main() {
//     let countdown = [5, 4, 3, 2, 1];
//     let number = countdown.get(2);
//     let number = match number {
//         Some(number) => number + 1,
//         None => 0,
//     };
//     println!("number is {:?}", number);
// }

// // option enum
// fn main() {
//     let countdown = [5, 4, 3, 2, 1];
//     let number = countdown.get(5);
//     let number = number.unwrap_or(&0) + 1;
//     println!("number is {:?}", number);
// }

// // enum methods
// #[derive(Debug)]
// enum Shape {
//     Circle(f64),             // radius
//     Rectangle(f64, f64),     // width, height
//     Triangle(f64, f64, f64), // sides a, b, c
// }

// impl Shape {
//     fn get_perimeter(&self) -> f64 {
//         match *self {
//             Shape::Circle(r) => r * 2.0 * std::f64::consts::PI,
//             Shape::Rectangle(w, h) => (2.0 * w) + (2.0 * h),
//             Shape::Triangle(a, b, c) => a + b + c,
//         }
//     }
// }

// fn main() {
//     let my_shape = Shape::Rectangle(1.2, 3.4);
//     println!("my_shape is {:?}", my_shape);

//     let perimeter = my_shape.get_perimeter();
//     println!("perimeter is {}", perimeter);
// }

// //default match
// fn main() {
//     let my_number = 3u8;

//     let result = match my_number {
//         0 => "zero",
//         _ => "one",
//         2 => "two",
//         3 => "three",
//         _ => {
//             println!("{} did not match", my_number);
//             "something else"
//         }
//     };
//     println!("result is {}", result);
// }

// // match operator
// #[derive(Debug)]
// enum Shape {
//     Circle(f64),             // radius
//     Rectangle(f64, f64),     // width, height
//     Triangle(f64, f64, f64), // sides a, b, c
// }

// fn main() {
//     let my_shape = Shape::Rectangle(1.2, 3.4);
//     println!("my_shape is {:?}", my_shape);

//     match my_shape {
//         Shape::Circle(r) => println!("Circle with radius {}", r),
//         Shape::Rectangle(w, h) => println!("{} x {} Rectangle", w, h),
//         Shape::Triangle(a, b, c) => println!("Triangle with sides {}, {}, {}", a, b, c),
//     }
// }

// // define enums
// #[derive(Debug)]
// enum Shape {
//     Circle(f64),             // radius
//     Rectangle(f64, f64),     // width, height
//     Triangle(f64, f64, f64), // sides a, b, c
// }

// fn main() {
//     let my_shape = Shape::Rectangle(1.2, 3.4);
//     println!("my_shape is {:?}", my_shape);
// }
