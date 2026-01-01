use std::io;

fn main() {
    println!("Enter n : ");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let n: u32 = input.trim().parse().expect("Please enter a valid number");

    let (mut a, mut b) = (0, 1);

    for _ in 0..n {
        print!("{} ", a);
        let temp = a + b;
        a = b;
        b = temp;
    }
    println!();
}
