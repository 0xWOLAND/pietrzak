#![no_main]
sp1_zkvm::entrypoint!(main);

use fibonacci_lib::fibonacci;

pub fn main() {
    let n = sp1_zkvm::io::read::<u32>();
    let (a, b) = fibonacci(n);
    // sp1_zkvm::io::commit_slice(&bytes);
}
