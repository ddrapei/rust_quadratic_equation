use std::io;
use std::io::Write;

fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        None // No real solutions
    } else {
        let sqrt_discriminant = discriminant.sqrt();
        let x1 = (-b + sqrt_discriminant) / (2.0 * a);
        let x2 = (-b - sqrt_discriminant) / (2.0 * a);
        Some((x1, x2))
    }
}

fn get_coefficient(prompt: &str) -> f64 {
    loop {
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().parse() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

fn main() {
    println!("Quadratic Equation Solver");
    println!("For equation in the form: ax² + bx + c = 0");

    let a = get_coefficient("Enter coefficient a: ");
    if a == 0.0 {
        println!("Error: 'a' cannot be zero as this would make the equation linear.");
        return;
    }

    let b = get_coefficient("Enter coefficient b: ");
    let c = get_coefficient("Enter coefficient c: ");

    match solve_quadratic(a, b, c) {
        Some((x1, x2)) => {
            if (x1 - x2).abs() < 1e-10 {
                println!("One real solution (double root): x = {:.4}", x1);
            } else {
                println!("Two real solutions:");
                println!("x₁ = {:.4}", x1);
                println!("x₂ = {:.4}", x2);
            }
        },
        None => println!("No real solutions (complex roots)"),
    }
}
