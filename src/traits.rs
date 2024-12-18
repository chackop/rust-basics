// display trait
use std::fmt;

struct Satellite {
    name: String,
    velocity: f64, // miles per second
}

impl fmt::Display for Satellite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} flying at {} miles per hour",
            self.name, self.velocity
        )
    }
}

fn main() {
    let hubble = Satellite {
        name: String::from("Hubble Telescope"),
        velocity: 4.72,
    };
    println!("hubble is {}", hubble);
}

// // return
// use std::fmt;

// fn get_displayable(choice: bool) -> impl fmt::Display {
//     if choice {
//         13
//     } else {
//         "thirteen"
//     }
// }

// fn main() {
//     println!("output is {}", get_displayable(true));
// }

// // multiple bounds
// use std::fmt;

// // fn compare_and_print<T: fmt::Display + PartialEq + From<U>, U: fmt::Display + PartialEq + Copy>(a: T, b: U) {
// fn compare_and_print<T, U>(a: T, b: U)
// where
//     T: fmt::Display + PartialEq + From<U>,
//     U: fmt::Display + PartialEq + Copy,
// {
//     if a == T::from(b) {
//         println!("{} is equal to {}", a, b);
//     } else {
//         println!("{} is NOT equal to {}", a, b);
//     }
// }

// fn main() {
//     compare_and_print(1.0, "one");
//     compare_and_print(1.1, 1);
// }

// // bounds
// use std::any;
// use std::fmt;

// fn print_type<T: fmt::Debug>(item: T) {
//     println!("{:?} is {}", item, any::type_name::<T>());
// }

// fn main() {
//     print_type(13);
//     print_type(13.0);
//     print_type("thirteen");
//     print_type([13]);
// }

// // derive
// #[derive(PartialEq, PartialOrd)]
// struct Satellite {
//     name: String,
//     velocity: f64, // miles per second
// }

// fn main() {
//     let hubble = Satellite {
//         name: String::from("Hubble Telescope"),
//         velocity: 4.72,
//     };
//     let gps = Satellite {
//         name: String::from("GPS"),
//         velocity: 2.42,
//     };
//     println!("hubble == gps is {}", hubble == gps);
//     println!("hubble > gps is {}", hubble > gps);
// }

// // default
// struct Satellite {
//     name: String,
//     velocity: f64, // miles per second
// }

// struct SpaceStation {
//     name: String,
//     crew_size: u8,
//     altitude: u32, // miles
// }

// trait Description {
//     fn describe(&self) -> String {
//         String::from("an object flying through space!")
//     }
// }

// impl Description for Satellite {}

// impl Description for SpaceStation {
//     fn describe(&self) -> String {
//         format!(
//             "the {} flying {} miles high with {} crew members aboard!",
//             self.name, self.altitude, self.crew_size
//         )
//     }
// }

// fn main() {
//     let hubble = Satellite {
//         name: String::from("Hubble Telescope"),
//         velocity: 4.72,
//     };
//     let iss = SpaceStation {
//         name: String::from("International Space Station"),
//         crew_size: 6,
//         altitude: 254,
//     };
//     println!("hubble is {}", hubble.describe());
//     println!("iss is {}", iss.describe());
// }

// // traits
// struct Satellite {
//     name: String,
//     velocity: f64, // miles per second
// }

// struct SpaceStation {
//     name: String,
//     crew_size: u8,
//     altitude: u32, // miles
// }

// trait Description {
//     fn describe(&self) -> String;
// }

// impl Description for Satellite {
//     fn describe(&self) -> String {
//         format!(
//             "the {} flying at {} miles per second!",
//             self.name, self.velocity
//         )
//     }
// }

// impl Description for SpaceStation {
//     fn describe(&self) -> String {
//         format!(
//             "the {} flying {} miles high with {} crew members aboard!",
//             self.name, self.altitude, self.crew_size
//         )
//     }
// }

// fn main() {
//     let hubble = Satellite {
//         name: String::from("Hubble Telescope"),
//         velocity: 4.72,
//     };
//     let iss = SpaceStation {
//         name: String::from("International Space Station"),
//         crew_size: 6,
//         altitude: 254,
//     };
//     println!("hubble is {}", hubble.describe());
//     println!("iss is {}", iss.describe());
// }
