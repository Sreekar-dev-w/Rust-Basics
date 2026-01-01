use std::io;

fn main() {
    let mut n = String::new();
    println!("Enter n  : ");
    io::stdin().read_line(&mut n).unwrap();
    let mut n: i32 = n.trim().parse().unwrap();

    let mut c = 0;
    let mut s = 0;
    let mut p = 1;
    while n > 0 {
        let d = n % 10;
        c += 1;
        s += d;
        p *= d;
        n /= 10;
    }
    println!("Sum = {}", s);
    println!("Count = {}", c);
    println!("Product = {}", p)
}
