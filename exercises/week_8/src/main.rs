use week_8::gcd;
use std::io;

fn main() {
    let mut a_str = String::new();
    let mut b_str = String::new();

    println!("Enter value of a:");
    io::stdin().read_line(&mut a_str).unwrap();
    let a = a_str.trim().parse::<u64>().unwrap();

    println!("Enter value of b:");
    io::stdin().read_line(&mut b_str).unwrap();
    let b = b_str.trim().parse::<u64>().unwrap();

    println!("The gcd of {} and {} is: {}", a, b, gcd(a, b));
}

#[cfg(test)]
fn test_gcd() {
    assert_eq!(gcd(10, 15), 5);
    assert_eq!(gcd(20, 30), 10);
    assert_eq!(gcd(15, 20), 5);
}