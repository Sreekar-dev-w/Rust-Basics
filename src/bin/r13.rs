fn main(){
    let mut c=0;

    loop{
        c+=1;
        println!("Count is : {}",c);

        if c==1000{
            break;
        }
    }
    println!("Loop Ended");
}