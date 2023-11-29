mod rsa;

use rsa::rsa_digital_signature;
use num_bigint::{BigUint, ToBigUint};

fn main() {
    let (public_key, n, private_key, _) = rsa_digital_signature::key_gen();

    println!("Public Key (e, n): ({}, {})", public_key, n);
    println!("Private Key (d, n): ({}, {})", private_key, n);
    // let n = &130040401u32.to_biguint().unwrap();
    // let big_prime = rsa_digital_signature::generate_large_prime();
    // println!("big prime: {}", big_prime);
    // println!("is {} prime: {}", n, rsa_digital_signature::is_prime(n));
    
}
