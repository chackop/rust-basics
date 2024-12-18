// struct lifetime
struct Shuttle<'a> {
    name: &'a str,
}

impl<'a, 'b> Shuttle<'a> {
    fn send_transmission(&'a self, msg: &'b str) -> &'b str {
        println!("Transmitting message: {}", msg);
        msg
    }
}

fn main() {
    let vehicle = Shuttle { name: "Endeavour" };

    let sender = vehicle.send_transmission("Greetings from orbit!");
    println!("sender is {}", sender);
}

// // lifetime elision
// fn main() {
//     let message = String::from("Greetings from Earth!");
//     let first_word = get_first_word(&message);
//     println!("first_word is {}", first_word);
// }

// fn get_first_word<'a>(s: &'a str) -> &'a str {
//     let bytes = s.as_bytes();

//     for (index, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..index]; // found a space!
//         }
//     }

//     &s // no spaces found; input is a single word
// }

// // multiple lifetime
// fn best_fuel<'a, 'b>(x: &'a str, y: &'b str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         x
//     }
// }

// fn main() {
//     let result;
//     let propellant1 = String::from("RP-1");
//     {
//         let propellant2 = String::from("LNG");
//         result = best_fuel(&propellant1, &propellant2);
//     }
//     println!("result is {}", result);
// }

// // annotations
// fn best_fuel<'a>(x: &'a str, y: &'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

// fn main() {
//     let result;
//     let propellant1 = String::from("RP-1");
//     {
//         let propellant2 = String::from("LNG");
//         result = best_fuel(&propellant1, &propellant2);
//     }
//     println!("result is {}", result);
// }

// // borrower checker
// fn main() {
//     let propellant;
//     {
//         let rp1 = String::from("RP-1");
//         propellant = &rp1;
//     }
//     println!("propellant is {}", propellant);
// }
