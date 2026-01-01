use std::io;
fn main() {
    let mut n = String::new();
    println!("Enter n : ");
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    let mut t = n;
    let mut r = 0;
    while t > 0 {
        let d = t % 10;
        r = r * 10 + d;
        t /= 10;
    }
    if r == n {
        println!("Palindrome");
    } else {
        println!("Not a palindrome");
    }
}
