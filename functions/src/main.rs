// fn main() {
//     println!("Hello, world!");

//     another_function();
// }

// fn another_function() {
//    println!("Another function!"); 
// }


// fn main() {
//     another_function(5, 6);
// }

// fn another_function(x: i32, y: i32) {
//    println!("The value of x is {}", x); 
//    println!("The value of y is {}", y); 
// }

// fn main() {
//     let x = 5;

//     let y = {
//         let x = 3;
//         x + 1
//     };

//     println!("The value of y is: {}", y);
// }


fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("The value of x is: {}", x);
}
