use std::io;
fn main(){
    let mut n=String::new();
    println!("Enter age : ");
    io::stdin().read_line(&mut n).expect("Failed to read");
    let n:i32 =n.trim().parse().unwrap();

    if n<=18 {
        println!("You are a minor");
        }else{
            println!("You are a adult");
        }
}
