// // Infinite Loop
// fn main() {
//     loop {
//         println!("again!");
//     }
// }

// // While Loop
// fn main() {
//     let mut number = 3;

//     while number != 0 {
//         println!("{}!", number);

//         number = number - 1;
//     }
    
//     println!("LIFT OFF!!!");
// }

// // For Loop
// fn main() {
//     let a = [10, 20, 30, 40, 50];

//     for element in a.iter() {
//         println!("The value is: {}", element);
//     }
// }

// Nicier For Loop
fn main() {
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    
    println!("LIFT OFF!!!");
}
