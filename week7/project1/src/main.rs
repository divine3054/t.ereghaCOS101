use std::io;


fn read_f64(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please enter a valid number")
}

// Area of Trapezium
fn area_trapezium(h: f64, b1: f64, b2: f64) -> f64 {
    h / 2.0 * (b1 + b2)
}

//Area of Rhombus
fn area_rhombus(d1: f64, d2: f64) -> f64 {
    0.5 * d1 * d2
}

//of Parallelogram
fn area_parallelogram(b: f64, a: f64) -> f64 {
    b * a
}

//Area of Cube (Surface Area)
fn area_cube(s: f64) -> f64 {
    6.0 * s.powi(2)
}

//Volume of Cylinder
fn volume_cylinder(r: f64, h: f64) -> f64 {
    3.142 * r.powi(2) * h
}

fn main() {
    println!("Select calculation:");
    println!("1. Trapezium Area");
    println!("2. Rhombus Area");
    println!("3. Parallelogram Area");
    println!("4. Cube Area");
    println!("5. Cylinder Volume");

    let choice = read_f64("Enter choice (1-5): ") as u32;

    match choice {
        1 => {
            let h = read_f64("Height:");
            let b1 = read_f64("Base:");
            let b2 = read_f64("Base 2:");
            println!("Result: {}", area_trapezium(h, b1, b2));
        }
        2 => {
            let d1 = read_f64("Diagonal 1:");
            let d2 = read_f64("Diagonal 2:");
            println!("Result: {}", area_rhombus(d1, d2));
        }
        3 => {
            let b = read_f64("Base:");
            let a = read_f64("Altitude:");
            println!("Result: {}", area_parallelogram(b, a));
        }
        4 => {
            let s = read_f64("Side length:");
            println!("Result: {}", area_cube(s));
        }
        5 => {
            let r = read_f64("Radius:");
            let h = read_f64("Height:");
            println!("Result: {}", volume_cylinder(r, h));
        }
        _ => println!("Invalid choice."),
    }
}