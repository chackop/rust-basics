// Trim spaces
fn main() {
    let test1 = "We need more space.";
    assert_eq!(trim_spaces(test1), "We need more space.");

    let test2 = String::from("   There's space in front.");
    assert_eq!(trim_spaces(&test2), "There's space in front.");

    let test3 = String::from("There's space to the rear. ");
    assert_eq!(trim_spaces(&test3[..]), "There's space to the rear.");

    let test4 = "  We're surrounded by space!    ";
    assert_eq!(trim_spaces(test4), "We're surrounded by space!");

    let test5 = "     ";
    assert_eq!(trim_spaces(test5), "");

    let test6 = "";
    assert_eq!(trim_spaces(test6), "");

    let test7 = " x🚀xx ";
    assert_eq!(trim_spaces(test7), "x🚀xx");
    println!("Tests passed!");
}

fn trim_spaces(s: &str) -> &str {
    // locate the first non-space character
    let mut start = 0;
    for (index, character) in s.chars().enumerate() {
        if character != ' ' {
            start = index;
            break;
        }
    }

    // search in reverse to locate the last non-space character
    let mut end = 0;
    for (index, character) in s.chars().rev().enumerate() {
        if character != ' ' {
            end = s.len() - index;
            break;
        }
    }

    &s[start..end]
}

// // slice params
// fn main() {
//     let message = String::from("Greetings from Earth!");
//     let first_word = get_first_word(&message);
//     println!("first_word is {}", first_word);
// }

// fn get_first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (index, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[..index]; // found a space!
//         }
//     }

//     &s // no spaces found; input is a single word
// }

// // slices
// fn main() {
//     let message = String::from("Greetings from Earth!");
//     println!("message is {}", message);

//     let last_word = &message[15..];
//     println!("last_word is {}", last_word);

//     let planets = [1, 2, 3, 4, 5, 6, 7, 8]; // sorry, Pluto!
//     let inner_planets: &[i32] = &planets[..4];
//     println!("inner_planets are {:?}", inner_planets);
// }

// // dangling refs
// fn main() {
//     let rocket_fuel = produce_fuel();
//     println!("rocket_fuel is {}", rocket_fuel);
// }

// fn produce_fuel() -> String {
//     let new_fuel = String::from("RP-1");
//     new_fuel
// }

// // Mut References
// fn main() {
//     let mut rocket_fuel = String::from("RP-1");
//     let length = process_fuel(&mut rocket_fuel);
//     println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
// }

// fn process_fuel(propellant: &mut String) -> usize {
//     println!("processing propellant {}...", propellant);
//     propellant.push_str(" is highly flammable!");
//     let length = propellant.len();
//     length
// }

// // Borrow refs
// fn main() {
//     let rocket_fuel = String::from("RP-1");
//     let length = process_fuel(&rocket_fuel);
//     println!("rocket_fuel is {} and length is {}", rocket_fuel, length);
// }

// fn process_fuel(propellant: &String) -> usize {
//     println!("processing propellant {}...", propellant);
//     let length = propellant.len();
//     length
// }
