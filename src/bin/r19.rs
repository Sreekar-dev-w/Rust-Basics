use std::io;

fn main() {
    let mut num = String::new();
    println!("Enter n : ");
    io::stdin().read_line(&mut num).expect("Failed to read input");
    let mut num: i32 = num.trim().parse().expect("Failed to convert to number");
    
    while num >= 10 {
        let mut sum = 0;
        let mut temp = num;

        while temp > 0 {
            sum += temp % 10;
            temp /= 10;
        }

        num = sum;
    }
    
    println!("Final single digit is: {}", num);
}
