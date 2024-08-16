#![no_main]
sp1_zkvm::entrypoint!(main);

use fibonacci_lib::timelock;
use num_bigint::BigUint;

pub fn main() {
    let x = sp1_zkvm::io::read_vec();
    let T = sp1_zkvm::io::read::<usize>();
    let modulus = BigUint::from_bytes_be(&sp1_zkvm::io::read_vec());

    let (intermediate_values, intermediate_powers) = timelock(&x, T, &modulus);

    for value in intermediate_values {
        sp1_zkvm::io::commit_slice(&value.to_bytes_be());
    }

    for value in intermediate_powers {
        sp1_zkvm::io::commit_slice(&value.to_bytes_be());
    }
}
