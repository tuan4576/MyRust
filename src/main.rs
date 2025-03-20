// fn main() {
//     println!("Hello, world!");
// }

// use std::io;

// fn main() {
//     println!("Please input your guess.");
//     let mut guess = String::new();
//     io::stdin()
//         .read_line(&mut guess)
//         .expect("Failed to read line");
//     println!("You guessed: {}", guess);
// }
fn main(){
    let tup = (10, 20, 30);
    let(x,y,z) = tup;
    let a = [1,2,3,4];
    // let a  = 10;
    // let a = a + 5;
    // let b = 20;
    // println!("Tổng là : {}", a + b);
    println!("Tổng là : {}",a[1]);
}