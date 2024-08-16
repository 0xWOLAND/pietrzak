use num_bigint::BigUint;
use num_traits::Num;
use sha2::{Digest, Sha224};

pub fn timelock(x: &[u8], T: usize, modulus: &BigUint) -> (Vec<BigUint>, Vec<BigUint>) {
    let g = sha224_to_biguint(x);

    let intermediate_values = (0..T).fold(vec![g.clone()], |mut acc, _| {
        acc.push(sha224_to_biguint(&acc.last().unwrap().to_bytes_be()));
        acc
    });

    let intermediate_powers = (0..T)
        .map(|i| {
            let exponent = BigUint::from(2u32).modpow(&BigUint::from(i + 1), modulus);
            g.modpow(&exponent, modulus)
        })
        .collect();

    (intermediate_values, intermediate_powers)
}

fn sha224_to_biguint(data: &[u8]) -> BigUint {
    let mut hasher = Sha224::new();
    hasher.update(data);
    let hash = hasher.finalize();
    let hex_string = hex::encode(hash);
    BigUint::from_str_radix(&hex_string, 16).expect("Failed to convert hex to BigUint")
}

#[test]
fn test_timelock() {
    let x = b"initial value";
    let T = 5;
    let modulus = BigUint::parse_bytes(
        b"FFFFFFFF00000001000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFF",
        16,
    )
    .unwrap(); // NIST P-256 curve order
    let (intermediate_values, intermediate_powers) = timelock(x, T, &modulus);

    println!("Intermediate values (H(X), H(H(X)), ...):");
    for (i, value) in intermediate_values.iter().enumerate() {
        println!("Step {}: {}", i, value);
    }

    println!("\nIntermediate powers (g^2, g^(2^2), g^(2^3), ...):");
    for (i, power) in intermediate_powers.iter().enumerate() {
        println!("Step {}: {}", i, power);
    }

    // Verify exponentiation
    let g = &intermediate_values[0]; // g = H(X)
    for (i, power) in intermediate_powers.iter().enumerate() {
        let expected_exponent = BigUint::from(2u32).modpow(&BigUint::from(i + 1), &modulus);
        let expected_power = g.modpow(&expected_exponent, &modulus);
        assert_eq!(
            power, &expected_power,
            "Mismatch at step {}. Expected: {}, Got: {}",
            i, expected_power, power
        );
        println!("Step {}: Exponentiation verified", i);
    }
    println!("All exponentiations verified successfully");
}
