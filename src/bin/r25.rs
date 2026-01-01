use std::io;

fn main() {
    let mut n = String::new();
    let mut fact = 1;
    println!("Enter n : ");
    io::stdin().read_line(&mut n).expect("Failed to read input");
    let n = n.trim().parse().expect("Failed to read input");

    for i in 1..=n {
        fact *= i;
    }
    println!("Factorial of {} is {} ", n, fact);
}
