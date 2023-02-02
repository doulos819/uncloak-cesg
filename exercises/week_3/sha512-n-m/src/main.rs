use openssl::hash::{Hasher, MessageDigest};
extern crate hex;
extern crate openssl;
extern crate rand;

use rand::Rng;

fn main() {
    // Create a Hasher instance for the SHA-512 hash algorithm.
    let mut hasher = Hasher::new(MessageDigest::sha512()).unwrap();

    // Set the target hash value in hexadecimal format.
    let target_hash = "3D4B";

    // Parse the target hash value from hexadecimal to bytes.
    let target_hash_bytes = hex::decode(target_hash).unwrap();

    // Set the number of bits in the hash value to use.
    let n = 16;

    // Set the maximum number of iterations to a large value.
    let max_iterations = 1_000_000;

    // Generate random messages and compute their hash values.
    for i in 0..max_iterations {
        // Generate a random message of length 8.
        let message = String::from_utf8(rand::thread_rng().sample_iter(&rand::distributions::Alphanumeric).take(8).collect::<Vec<u8>>()).unwrap();

        // Hash the message.
        hasher.update(message.as_bytes()).unwrap();

        // Get the hash value of the message.
        let hash = hasher.finish().unwrap();

        // Check if the first n bits of the hash value match the target hash value.
        if hash[..n / 8] == target_hash_bytes[..] {
            // If the first n bits of the hash value match the target hash value, output the message and its hash value.
            println!("Message found after: {} iterations", i);
            println!("Message: {}", message);
            println!("Hash value: {}", hex::encode(&hash[..n / 8]));
            break;
        }
    }
}

