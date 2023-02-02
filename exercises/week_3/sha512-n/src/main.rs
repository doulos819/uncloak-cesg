use std::collections::HashMap;
use openssl::hash::{Hasher, MessageDigest};
extern crate hex;
extern crate openssl;
extern crate rand;

use rand::Rng;

fn main() {
    // Create a Hasher instance for the SHA-512 hash algorithm.
    let mut hasher = Hasher::new(MessageDigest::sha512()).unwrap();

    // Create a map to store the hashes of the input strings.
    let mut hash_map: HashMap<Vec<u8>, Vec<u8>> = HashMap::new();

    // Set the number of bits in the hash value to use for the birthday attack.
    let n = 8;

    // Set the maximum number of iterations to a large value.
    let max_iterations = 1_000_000;

    // Generate random input strings and compute their hash values.
    for i in 0..max_iterations {
        // Generate a random input string of length n / 8.
        let input = String::from_utf8(rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(n / 8).collect::<Vec<u8>>()).unwrap().into_bytes();

        // Hash the input string.
        hasher.update(&input).unwrap();

        // Get the hash value of the input string.
        let hash = hasher.finish().unwrap();

        // Check if the hash value has already been seen.
        if let Some(input_string) = hash_map.get(&hash.as_ref()[..n / 8]) { // think about making this a variable
            // If the hash value has been seen before, output the collision information.
            println!("Collision found after {} iterations:", i);
            println!("Input string 1: {}", String::from_utf8(input.clone()).unwrap());
            println!("Input string 2: {}", String::from_utf8(input_string.clone()).unwrap());
            println!("Hash value 1: {}", hex::encode(&hash[..n / 8]));
            println!("Hash value 2: {}", hex::encode(&hash_map[&hash.as_ref()[..n / 8]]));
            break;
        }

        // Store the hash value in the map.
        hash_map.insert(Vec::from(&hash[..n / 8]), input);
    }
}