use std::io;
fn main(){
    let mut n =String::new();
    println!("Enter marks : ");
    io::stdin().read_line(&mut n).unwrap();
    let n:i32=n.trim().parse().unwrap();

    let grade = match n{
        90..=100 =>"A",
        81..=91 => "B",
        60..=79 => "C",
        50..=70 => "D",
        0..=49 => "Fail 😭",
        _ => "Invalid  Marks 😭"

    };
    println!("Grade = {}",grade);
}