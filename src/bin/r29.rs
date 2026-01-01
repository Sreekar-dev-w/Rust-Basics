use std::io;
fn main() {
    let mut n = String::new();
    println!("Enter n  : ");
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    let mut t = n;
    let mut s = 0;
    while t > 0 {
        let d = t % 10;
        let mut f = 1;
        let mut i = 1;

        while i <= d {
            f *= i;
            i += 1;
        }
        s += f;
        t /= 10;
    }
    if s == n {
        println!("Strong number");
    } else {
        println!("Not a strong number");
    }
}
