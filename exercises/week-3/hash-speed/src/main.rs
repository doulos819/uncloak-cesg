extern crate openssl;

use openssl::hash::{Hasher, MessageDigest};
use openssl::sha;


fn main() {
    let message = "I dislike bench testing";

    let mut hasher2 = Hasher::new(MessageDigest::sha256()).unwrap();
    hasher2.update(message.as_bytes()).unwrap();
    let sha2_256 = hasher2.finish().unwrap();

    let mut hasher3 = Hasher::new(MessageDigest::sha3()).unwrap();
    hasher3.update(message.as_bytes()).unwrap();
    let sha3_256 = hasher3.finish().unwrap();

    println!("The sha2-256 hash of the message is: {}", sha2::Sha256::from_hash(sha2_256).to_hex());
    println!("The sha3-256 hash of the message is: {}", sha3::Sha3_256::from_hash(sha3_256).to_hex());
}
