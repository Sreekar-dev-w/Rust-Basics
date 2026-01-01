fn main(){
    for i in 1..101{
        if i%2 != 0{
            continue;
        }else{
            println!("{}",i);
        }
    }
}
// This program skips odd numbers from 1 too 100 and prints only even numbers if u want only odd numbers to be printed then change ! to = in if statement