use week_10::{fast_power, find_generator};
use std::io;
use num_bigint::BigInt;


fn main() {
    // // Declare a new string to store the base value
    // let mut base_str = String::new();

    // // Declare a new string to store the exponent value
    // let mut exponent_str = String::new();

    // // Declare a new string to store the modulus value
    // let mut modulus_str = String::new();

    // // Prompt the user to enter the base value
    // println!("Enter the base:");

    // // Read the user input into the `base_str` string
    // io::stdin().read_line(&mut base_str).unwrap();

    // // Parse the `base_str` string into a BigInt value
    // let base = BigInt::parse_bytes(base_str.trim().as_bytes(), 10).unwrap();

    // // Prompt the user to enter the exponent value
    // println!("Enter the exponent:");

    // // Read the user input into the `exponent_str` string
    // io::stdin().read_line(&mut exponent_str).unwrap();

    // // Parse the `exponent_str` string into a BigInt value
    // let exponent = BigInt::parse_bytes(exponent_str.trim().as_bytes(), 10).unwrap();

    // // Prompt the user to enter the modulus value
    // println!("Enter the modulus:");

    // // Read the user input into the `modulus_str` string
    // io::stdin().read_line(&mut modulus_str).unwrap();

    // // Parse the `modulus_str` string into a BigInt value
    // let modulus = BigInt::parse_bytes(modulus_str.trim().as_bytes(), 10).unwrap();

    // // Calculate and print the result of `fast_power` with the base, exponent, and modulus values
    // println!("The result is: {}", fast_power(base, exponent, modulus));

    println!("Enter a prime number: ");
    let mut prime_input = String::new();
    io::stdin().read_line(&mut prime_input).unwrap();
    let prime: BigInt = prime_input.trim().parse().unwrap();

    match find_generator(&prime) {
        Some(generator) => println!("A generator for {} is: {}", prime, generator),
        None => println!("No generator was found for {}", prime),
    }

}


