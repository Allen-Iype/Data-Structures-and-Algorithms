use std::io;

fn is_prime(n: u32) -> bool {
    let mut divisor = 2;

    while n > divisor {
        if n % divisor == 0 {
            return false;
        } else {
            divisor += 1;
        }
    }
    true
}

fn main() {
    println!("Enter a number:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input.");

    let input: u32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input. Please enter a valid number.");
            return;
        }
    };

    if is_prime(input) {
        println!("{} is a prime number.", input);
    } else {
        println!("{} is not a prime number.", input);
    }
}