enum TrafficLight {
    Red,
    Yellow,
    Green,
}

use std::io;

fn main() {
    let mut input = String::new();
    println!("Enter signal (red / yellow / green): ");
    io::stdin().read_line(&mut input).unwrap();

    let input = input.trim().to_lowercase(); 

    let signal = match input.as_str() {
        "red" => TrafficLight::Red,
        "yellow" => TrafficLight::Yellow,
        "green" => TrafficLight::Green,
        _ => {
            println!("Invalid signal!");
            return;
        }
    };

    match signal {
        TrafficLight::Red => println!("STOP"),
        TrafficLight::Yellow => println!("WAIT"),
        TrafficLight::Green => println!("GO"),
    }
}
// copy of r8 but agaiin for practice