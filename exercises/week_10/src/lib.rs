use std::ops::Mul;
use num_bigint::BigInt;
use num_traits::Zero;

pub fn fast_power(base: BigInt, mut exponent: BigInt, modulus: BigInt) -> BigInt {
    // Create a zero BigInt to compare the exponent to.
    let zero = BigInt::zero();

    // Create a one BigInt for the result.
    let one = BigInt::from(1u8);

    // Initialize result to one.
    let mut result = one.clone();

    // Clone the base to avoid modifying the original.
    let mut base = base.clone();

    // Loop while exponent is greater than zero.
    while exponent > zero {
        // If the exponent is odd, multiply result by base and take the modulus.
        if &exponent % 2u8 == one {
            result = (result.clone() * base.clone()) % modulus.clone();
        }

        // Square the base and take the modulus.
        base = (base.clone() * base.clone()) % modulus.clone();

        // Right-shift the exponent by 1 bit.
        exponent >>= 1;
    }

    // Return the result.
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigInt;
    use num_traits::Zero;

    #[test]
    fn test_fast_power() {
        // Test 1: Check if (2^3) % 5 == 3
        let base = BigInt::from(2u8);
        let exponent = BigInt::from(3u8);
        let modulus = BigInt::from(5u8);
        let expected_result = BigInt::from(3u8);

        let result = fast_power(base, exponent, modulus);
        assert_eq!(result, expected_result);

        // Test 2: Check if (3^4) % 7 == 2
        let base = BigInt::from(3u8);
        let exponent = BigInt::from(4u8);
        let modulus = BigInt::from(7u8);
        let expected_result = BigInt::from(4u8);

        let result = fast_power(base, exponent, modulus);
        assert_eq!(result, expected_result);
    }
}
