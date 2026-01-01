use std::io;

fn main() {
    let mut n = String::new();
    println!("Enter a number: ");

    io::stdin().read_line(&mut n).unwrap();

    let mut n: i32 = n.trim().parse().unwrap();
    let mut t = 0;

    while n > 0 {
        t = t * 10 + n % 10;
        n = n / 10;
    }

    println!("Reverse = {} ", t);
}
