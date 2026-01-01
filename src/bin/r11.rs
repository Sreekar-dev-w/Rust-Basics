use std::io;
enum Shape{
    Circle(f64),
    Square(f64,f64),
    Rectangle(f64),
}
fn main(){
    println!("Choose Shape : ");
    println!("1.Circle");
    println!("2.Rectangle");
    println!("3.Square");

    let mut c = String::new();
    io::stdin().read_line(&mut c).unwrap();
    let c = c.trim();

    let s = if c=="1"{
        let mut r = String::new();
        println!("Enter radius : ");
        io::stdin().read_line(&mut r).expect("Failed to read input");
        let r:f64 = r.trim().parse().unwrap();
        Shape::Circle(r)
}else if c=="2"{
    let mut l = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut l).unwrap();
    io::stdin().read_line(&mut b).unwrap();

    let l:f64 = l.trim().parse().expect("Fail");
    let b:f64 = b.trim().parse().expect("Feiled to read input");
    Shape::Square(l, b)
}else if c=="3"{


    println!("Enter side : ");
    let mut w = String::new();

    io::stdin().read_line(&mut w).unwrap();
    

    let w:f64 = w.trim().parse().unwrap();
    
    Shape::Rectangle(w)
}
else{
    println!("Invalid input");
    return;
};
match s{
    Shape::Circle(r) => {
        println!("Area = {}",3.14*r*r);
    }
    Shape::Square(l,b) => {
        println!("Area = {}",l*b);
    }
    Shape::Rectangle(w) => {
        println!("Area = {}", w*w);
    }
}
}
// This is the code it takes input of your choice and finds area according to the input;