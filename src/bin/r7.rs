use std::io;
fn main(){
    let mut n =String::new();
    println!("Enter a number");

    io::stdin().read_line(&mut n).unwrap();
    let  n:i32 =n.trim().parse().unwrap();

  if n % 3 == 0 && n % 5 == 0 {
        println!("FizzBuzz💀");
    } else if n % 3 == 0 {
        println!("Fizz💀");
    } else if n % 5 == 0 {
        println!("Buzz💀");
    } else {
        println!("{}", n);
    }
}