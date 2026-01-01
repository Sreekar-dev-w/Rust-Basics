use std::io;

fn main(){
    let mut n = String::new();

    let mut sum: i32 =0;
    
     println!("Enter n : ");
     io::stdin().read_line(&mut n).unwrap();
     

     let  n:i32 = n.trim().parse().unwrap();

     for i in 1..n{
        sum+=i;
     }
     println!("Sum is : {}",sum);

}