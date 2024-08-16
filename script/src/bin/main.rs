use num_bigint::BigUint;
use sp1_sdk::{ProverClient, SP1Stdin};
use std::str::FromStr;

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

fn main() {
    sp1_sdk::utils::setup_logger();
    let client = ProverClient::new();
    let mut stdin = SP1Stdin::new();

    let x = "Initial data".as_bytes();
    let T: usize = 5;
    let modulus = BigUint::from_str(
        "21888242871839275222246405745257275088548364400416034343698204186575808495617",
    )
    .unwrap()
    .to_bytes_be();

    stdin.write(&x);
    stdin.write(&T);
    stdin.write(&modulus);

    let (mut output, report) = client.execute(ELF, stdin).run().unwrap();

    println!("Intermediate values (H(X), H(H(X)), ...):");
    for _ in 0..T {
        let mut buf: [u8; 28] = [0; 28];
        output.read_slice(&mut buf);
        let value = BigUint::from_bytes_be(&buf);
        println!("Step: {}", value);
    }

    println!("\nIntermediate powers (g^2, g^(2^2), g^(2^3), ...):");
    for _ in 0..T {
        let mut buf: [u8; 28] = [0; 28];
        output.read_slice(&mut buf);
        let value = BigUint::from_bytes_be(&buf);
        println!("Step: {}", value);
    }

    println!("Program executed successfully.");
}
