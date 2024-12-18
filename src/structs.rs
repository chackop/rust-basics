// shapes
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn get_area(&self) -> f64 {
        self.width * self.height
    }

    fn scale(&mut self, scalar: f64) {
        self.width *= scalar;
        self.height *= scalar;
    }

    fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
}

fn main() {
    let mut rect = Rectangle::new(1.2, 3.4);
    assert_eq!(rect.get_area(), 4.08);
    rect.scale(0.5);
    assert_eq!(rect.get_area(), 1.02);
    println!("Tests passed!");
}

// // tuple structs
// struct Color(u8, u8, u8); // RGB
// struct Point(u8, u8, u8); // XYZ

// fn get_y(p: Point) -> u8 {
//     p.1
// }

// fn main() {
//     let red = Color(255, 0, 0);
//     println!("First value is {}", red.0);

//     let coord = Point(4, 5, 6);
//     let y = get_y(coord);
//     println!("y is {}", y);
// }

// // associated functions
// struct Shuttle {
//     name: String,
//     crew_size: u8,
//     propellant: f64,
// }

// impl Shuttle {
//     fn get_name(&self) -> &str {
//         &self.name
//     }

//     fn add_fuel(&mut self, gallons: f64) {
//         self.propellant += gallons;
//     }

//     fn new(name: &str) -> Shuttle {
//         Shuttle {
//             name: String::from(name),
//             crew_size: 7,
//             propellant: 0.0,
//         }
//     }
// }

// fn main() {
//     let mut vehicle = Shuttle::new("Endeavour");
//     let mut vehicle2 = Shuttle::new("Discovery");

//     let vehicle_name = vehicle.get_name();
//     println!("vehicle_name is {}", vehicle_name);

//     println!("propellant is {}", vehicle.propellant);
//     vehicle.add_fuel(1000.0);
//     println!("propellant is {}", vehicle.propellant);
// }

// // struct methods
// struct Shuttle {
//     name: String,
//     crew_size: u8,
//     propellant: f64,
// }

// impl Shuttle {
//     fn get_name(&self) -> &str {
//         &self.name
//     }

//     fn add_fuel(&mut self, gallons: f64) {
//         self.propellant += gallons;
//     }
// }

// fn main() {
//     let mut vehicle = Shuttle {
//         name: String::from("Endeavour"),
//         crew_size: 7,
//         propellant: 0.0,
//     };

//     let vehicle_name = vehicle.get_name();
//     println!("vehicle_name is {}", vehicle_name);

//     println!("propellant is {}", vehicle.propellant);
//     vehicle.add_fuel(1000.0);
//     println!("propellant is {}", vehicle.propellant);
// }

// update structs
// #[derive(Debug, Clone)]
// struct Shuttle {
//     name: String,
//     crew_size: u8,
//     propellant: f64,
// }

// fn main() {
//     let mut vehicle = Shuttle {
//         name: String::from("Endeavour"),
//         crew_size: 7,
//         propellant: 835958.0,
//     };

//     let vehicle2 = Shuttle { ..vehicle.clone() };

//     vehicle.crew_size = 6;
//     println!("vehicle is {:?}", vehicle);
//     println!("vehicle2 is {:?}", vehicle2);
// }

// // define struct
// #[derive(Debug)]
// struct Shuttle {
//     name: String,
//     crew_size: u8,
//     propellant: f64,
// }

// fn main() {
//     let mut vehicle = Shuttle {
//         name: String::from("Endeavour"),
//         crew_size: 7,
//         propellant: 835958.0,
//     };
//     println!("name is {}", vehicle.name);

//     vehicle.name = String::from("Atlantis");
//     println!("vehicle is {:?}", vehicle);
// }
