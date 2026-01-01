fn main(){
    for i in 1..101{
        if i%5==0{
            break;
        }else{
            println!("{}",i);
        }
    }
}