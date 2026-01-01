use std::io;

fn main() {
    let mut n = String::new();
    println!("Enter n: ");
    io::stdin().read_line(&mut n).unwrap();
    let n: i32 = n.trim().parse().unwrap();

    let mut t = n;
    let mut count = 0;
    let mut sum = 0;
    let mut prod = 1;
    let mut rev = 0;
    let mut strong_sum = 0;

    while t > 0 {
        let d = t % 10;
        count += 1;
        sum += d;
        prod *= d;
        rev = rev * 10 + d;

        // factorial part for strong number
        let mut fact = 1;
        let mut i = 1;
        while i <= d {
            fact *= i;
            i += 1;
        }
        strong_sum += fact;

        t /= 10;
    }

    println!("Digits: {}", count);
    println!("Sum: {}", sum);
    println!("Product: {}", prod);
    println!("Reverse: {}", rev);

    if rev == n {
        println!("Palindrome: Yes");
    } else {
        println!("Palindrome: No");
    }

    if strong_sum == n {
        println!("Strong number: Yes");
    } else {
        println!("Strong number: No");
    }
}
