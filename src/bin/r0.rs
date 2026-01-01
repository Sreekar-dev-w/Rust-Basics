use std::io;
fn main(){
    let mut n = String::new();
    println!("Enter first number : ");
    io::stdin().read_line(&mut n).expect("Failed to read");
    let n:i32 = n.trim().parse().expect("Invalid number");

    let mut m = String::new();
    println!("Enter second number : ");
    io::stdin().read_line(&mut m).expect("Failed to read");
    let m:i32 = m.trim().parse().expect("Invalid input");

    println!("Sum : {}",m+n);
    println!("Diff :{}",n-m);
    println!("Product : {}",m*n);
    println!("Division : {}",n/m);
    println!("Modulo : {}",n%m);
}