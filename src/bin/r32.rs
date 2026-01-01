use std::io;

fn main() {
    let mut n = String::new();
    println!("Enter a number : ");
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().expect("Failed to read input");
    println!("Remainder is : {}", n % 10);
}
