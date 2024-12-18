// Transfer ownership
// fn main() {
//     let rocket_fuel = String::from("RP-1");
//     let rocket_fuel = process_fuel(rocket_fuel);
//     println!("rocket_fuel is {}", rocket_fuel);
// }

// fn process_fuel(propellant: String) -> String {
//     println!("processing propellant {}...", propellant);
//     let new_fuel = String::from("LNG");
//     new_fuel
// }

// move, clone, copy
// fn main() {
//     let outer_planet: i32;
//     {
//         let mut inner_planet = 1;
//         outer_planet = inner_planet;
//         inner_planet += 1;
//         println!("inner_planet is {}", inner_planet);
//     }
//     println!("outer_planet is {}", outer_planet);
// }

// String
// fn main() {
//     let mut message = String::from("Earth");
//     println!("message is {}", message);
//     message.push_str(" is home.");
//     println!("message is {}", message);
// }

// shadowing
// fn main() {
//     let planet = "Earth";
//     {
//         println!("planet is {}", planet);
//         let mut planet = 4;
//         println!("planet is {}", planet);
//     }
//     println!("planet is {}", planet);
// }

// scope
// fn main() {
//     let planet = "Earth";
//     if true {
//         println!("planet is {}", planet);
//     }
//     println!("planet is {}", planet);
// }
