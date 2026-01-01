use std::io;
fn main(){
    let mut a =String::new();
    let mut b=String::new();

    println!("Enter 2 numbers : ");
    io::stdin().read_line(&mut a).expect("Faile to read 😭");
    io::stdin().read_line(&mut b).expect("Invalid input 😭");

    let a:i32=a.trim().parse().expect("Invalid 😭");
    let b:i32=b.trim().parse().expect("Fail 😭");

    if b>a{
        println!("Max is : {}",b);
    }else if b<a{
        println!("Max is : {}",a);
    }else{
        println!("Both are equal 🫡");
    }   

        } 


