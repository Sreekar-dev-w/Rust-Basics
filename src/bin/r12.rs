use std::io; 
enum Operation {
    Add(f64, f64),
    Sub(f64, f64),
    Mul(f64, f64),
    Div(f64, f64),
}

fn main() {
    println!("Simple Calculator:");
    println!("1. Add 🫡");
    println!("2. Subtract 🫡");
    println!("3. Multiply 🫡");
    println!("4. Divide 🫡");

    let mut op_choice = String::new();
    io::stdin().read_line(&mut op_choice).unwrap();
    let op_choice = op_choice.trim();

    println!("Enter two numbers  :");

    let mut x = String::new();
    let mut y = String::new();

    io::stdin().read_line(&mut x).unwrap();
    io::stdin().read_line(&mut y).unwrap();

    let x: f64 = x.trim().parse().unwrap();
    let y: f64 = y.trim().parse().unwrap();

    // Create enum value depending on user choice
    let calc = 
    if op_choice == "1" {
        Operation::Add(x, y)
    } else if op_choice == "2" {
        Operation::Sub(x, y)
    } else if op_choice == "3" {
        Operation::Mul(x, y)
    } else if op_choice == "4" {
        Operation::Div(x, y)
    } else {
        println!("Invalid choice");
        return;
    };

    // Pattern match and perform operation this is the main part

    match calc {
        Operation::Add(a, b) => println!("Result: {} 🫡", a + b),

        Operation::Sub(a, b) => println!("Result: {} 🫡", a - b),

        Operation::Mul(a, b) => println!("Result: {} 🫡", a * b),

        Operation::Div(a, b) => {
            if b == 0.0 {
                println!("Error: Division by zero    🫡");
            } else {
                println!("Result: {} 🫡", a / b);
            }
        }
    }
}
