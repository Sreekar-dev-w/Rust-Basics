use std::io;

fn main(){
    let mut n=2;
    let mut t = String::new();
    println!("Enter n : ");
    io::stdin().read_line(&mut t).unwrap();
    let t = t.trim().parse().unwrap();
    while n<=t{
        println!("{} ",n);
        n+=2;
    }
}
// this priints even numbers between the user entered number and 2