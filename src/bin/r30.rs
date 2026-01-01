use std::io;

fn main() {
    let mut n = String::new();
    println!("Enter number of rows: ");
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    let mut i = 1;
    while i <= n {
        let mut space = n - i;
        while space > 0 {
            print!("  ");
            space -= 1;
        }

        let mut j = 1;
        while j <= i {
            print!("{} ", j);
            j += 1;
        }

        let mut k = i - 1;
        while k >= 1 {
            print!("{} ", k);
            k -= 1;
        }

        println!();
        i += 1;
    }
}
