use std::io;

fn main(){
    let mut n = String::new();

    println!("Enter number : ");
    io::stdin().read_line(&mut n).unwrap();

    let  mut n:i32 = n.trim().parse().unwrap();
    
    loop {
        println!("{}",n);
        n+=1;
        if n>100{
            break;
        }

    }
}