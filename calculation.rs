use std::f64::consts::PI;
use std::io;

enum Shape {
    Circle(f64), // Fix spelling from Circole to Circle
    Rectangle(f64, f64),
    Triangle(f64, f64),
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => PI * radius * radius, // Fix logic here for Circle area
            Shape::Rectangle(w, h) => w * h,
            Shape::Triangle(b, h) => 0.5 * b * h,
        }
    }
}

fn main() {
    loop {
        println!("Simple Mathematics Calculator for Students 🧑‍🎓");
        println!("--------------------------------------------");
        println!("Enter command: \n 1: Circle \n 2: Rectangle \n 3: Triangle \n q: for exit");
        let first = input().trim().to_string(); // Trim whitespace and newline characters
        match first.as_str() {
            "1" => {
                println!("Enter radius size:");
                let num1 = input().trim().parse::<f64>();
                match num1 {
                    Ok(radius) => println!("The result is: {}", Shape::Circle(radius).area()),
                    Err(_) => println!("Invalid input for radius. Please enter a valid number."),
                }
            }

            "2" => {
                println!("Enter width size:");
                let num1 = input().trim().parse::<f64>();
                println!("Enter height size:");
                let num2 = input().trim().parse::<f64>();
                match (num1, num2) {
                    (Ok(width), Ok(height)) => {
                        println!("The result is: {}", Shape::Rectangle(width, height).area())
                    }
                    _ => println!("Invalid input for width or height. Please enter valid numbers."),
                }
            }

            "3" => {
                println!("Enter base size:");
                let num1 = input().trim().parse::<f64>();
                println!("Enter height size:");
                let num2 = input().trim().parse::<f64>();
                match (num1, num2) {
                    (Ok(base), Ok(height)) => {
                        println!("The result is: {}", Shape::Triangle(base, height).area())
                    }
                    _ => println!("Invalid input for base or height. Please enter valid numbers."),
                }
            }
            "q" => break,
            _ => println!("Please Enter a valid command."),
        }
    }
}

fn input() -> String {
    let mut s = String::new();
    io::stdin().read_line(&mut s).expect("Failed to read line");
    s
}
