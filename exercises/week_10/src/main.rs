use week_10::{fast_power, find_generator};
use std::io;
use num_bigint::BigInt;


fn main() {
    // Declare a new string to store the base value
    let mut base_str = String::new();

    // Declare a new string to store the exponent value
    let mut exponent_str = String::new();

    // Declare a new string to store the modulus value
    let mut modulus_str = String::new();

    // Prompt the user to enter the base value
    println!("Enter the base:");

    // Read the user input into the `base_str` string
    io::stdin().read_line(&mut base_str).unwrap();

    // Parse the `base_str` string into a BigInt value
    let base = BigInt::parse_bytes(base_str.trim().as_bytes(), 10).unwrap();

    // Prompt the user to enter the exponent value
    println!("Enter the exponent:");

    // Read the user input into the `exponent_str` string
    io::stdin().read_line(&mut exponent_str).unwrap();

    // Parse the `exponent_str` string into a BigInt value
    let exponent = BigInt::parse_bytes(exponent_str.trim().as_bytes(), 10).unwrap();

    // Prompt the user to enter the modulus value
    println!("Enter the modulus:");

    // Read the user input into the `modulus_str` string
    io::stdin().read_line(&mut modulus_str).unwrap();

    // Parse the `modulus_str` string into a BigInt value
    let modulus = BigInt::parse_bytes(modulus_str.trim().as_bytes(), 10).unwrap();

    // Calculate and print the result of `fast_power` with the base, exponent, and modulus values
    println!("The result is: {}", fast_power(base, exponent, modulus));

    // Prompt user to enter the first prime
    println!("Enter the first prime number: ");
    // Declare prime1 as a new String
    let mut prime1 = String::new();
    // Read the user input into prime1
    io::stdin().read_line(&mut prime1).unwrap();
    // Parse 
    let prime1 = BigInt::parse_bytes(prime1.trim().as_bytes(), 10).unwrap();

    // Ask user to enter the second prime
    println!("Enter the second prime number: ");
    let mut prime2 = String::new();
    //Read the user input into prime2
    io::stdin().read_line(&mut prime2).unwrap();
    // Parse
    let prime2 = BigInt::parse_bytes(prime2.trim().as_bytes(), 10).unwrap();

    //
    let generator1 = find_generator(&prime1);
    let generator2 = find_generator(&prime2);
    match (generator1, generator2) {
    (Some(g1), Some(g2)) if g1 == g2 => println!("The generator is: {}", g1),
    (_, _) => println!("No common generator found"),
}

}

