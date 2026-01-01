use std::io;
fn main(){
    let mut n=String::new();
    println!("Enter Marks : ");
    io::stdin().read_line(&mut n).unwrap();
    let n:i32=n.trim().parse().unwrap();
    if n>=90{
        println!("Grade = 'A'");
    }else if n>=80{
        println!("Grade = 'B'");
    }else if n>=70{
        println!("Grade = 'C'");
    }else if n>=50 {
        println!("Grade = 'D'");
    }else{
        println!("Fail");
    }
}