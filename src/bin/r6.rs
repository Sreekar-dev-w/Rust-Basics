use std::io;
fn main(){
    let mut n=String::new();
    println!("Enter a Year : ");
    io::stdin().read_line(&mut n).unwrap();
    let n:i32 =n.trim().parse().unwrap();

    if(n%400==0)||(n%4==0 && n%100!=0){
        println!("Leap Year 🥳🥳🥳");
    }else{
        println!("Not a Leap Year 😭😭😭");
    }
}