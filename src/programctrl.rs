// maxminmean
// fn main() {
//     let numbers = [1, 9, -2, 0, 23, 20, -7, 13, 37, 20, 56, -18, 20, 3];
//     let mut max: i32;
//     let mut min: i32;
//     let mut mean: f64;

//     max = numbers[0];
//     min = numbers[0];
//     mean = 0.0;

//     for &num in numbers.iter() {
//         if num > max {
//             max = num;
//         } else if num < min {
//             min = num;
//         }
//         mean += num as f64;
//     }
//     mean /= numbers.len() as f64;

//     assert_eq!(max, 56);
//     assert_eq!(min, -18);
//     assert_eq!(mean, 12.5);
//     println!("Tests passed!");
// }

// nested loop
// fn main() {
//     let mut matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];

//     for row in matrix.iter_mut() {
//         for num in row.iter_mut() {
//             *num += 10;
//             print!("{}\t", num);
//         }
//         println!();
//     }
// }

// for loop
// fn main() {
//     let message = ['h', 'e', 'l', 'l', 'o'];

//     for (index, &item) in message.iter().enumerate() {
//         println!("item {} is {}", index, item);
//         if item == 'e' {
//             break;
//         }
//     }

//     for number in 0..5 {
//         println!("number is {}", number);
//     }
// }

// while loop
// fn main() {
//     let mut count = 0;
//     let letters = ['a', 'b', 'c'];

//     while count < letters.len() {
//         println!("letter is {}", letters[count]);
//         count += 1;
//     }
// }

// loops
// fn main() {
//     let mut count = 0;

//     let result = loop {
//         if count == 10 {
//             break count * 10;
//         }
//         count += 1;
//         println!("count is {}", count);
//     };

//     println!("After the loop!");
//     println!("result is {}", result);
// }

// conditional assign
// fn main() {
//     let make_x_odd = true;
//     let x = if make_x_odd { 1 } else { 2.0 };

//     /* if make_x_odd {
//         x = 1;
//     } else {
//         // x = 2;
//     } */
//     println!("x is {}", x);
// }

// multi condition
// fn main() {
//     let x = 3;
//     let y = 5;

//     if x > y {
//         println!("x is greater than y");
//     } else {
//         if x < y {
//             println!("x is less than y");
//         } else {
//             println!("x is equal to y");
//         }
//     }

//     if x > y {
//         println!("x is greater than y");
//     } else if x < y {
//         println!("x is less than y");
//     } else {
//         println!("x is equal to y");
//     }
// }

// conditional exec
// fn main() {
//     let x = 4;

//     if x + 1 != 3 {
//         println!("x + 1 is NOT 3!");
//     }
// }
