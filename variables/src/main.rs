// fn main() {
//     //let x = 5 would cause a mutability problem
//     let mut x = 5;
//     println!("The value of x is {}",x);
//     x = 6;
//     println!("The value of x is {}",x);
// }

fn main() {
    let x = 5;

    let x = x + 1;

    let x = x + 2;

    println!("The valu of x is {}", x);
}