enum S{
    Red,
    Yellow,
    Green,
}

fn main(){
    let x=S::Green;
    let _y=S::Red;
    let _z=S::Yellow;
    match x{
        S::Red => println!("Stop"),
        S::Yellow => println!("Wait"),
        S::Green => println!("GOOOOOOOOOOOOOOOO 💀💀💀"),
    }
}