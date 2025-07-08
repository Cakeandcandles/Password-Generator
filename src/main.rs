use rand::Rng;
use std::io;

fn main() {
    println!("Enter password length:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let length: usize = match input.trim().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("Please enter a valid number.");
            return;
        }
    };

    let charset: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@#$%^&*()-_=+[]{}"
        .chars()
        .collect();

    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..charset.len());
            charset[idx]
        })
        .collect();

    println!("Generated password: {}", password);
}
