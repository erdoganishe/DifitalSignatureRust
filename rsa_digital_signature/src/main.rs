mod rsa;

use rsa::rsa_digital_signature::{key_gen, hash_message, sign, verify};
use num_bigint::{BigUint, ToBigUint};

fn main() {
    // Key generation
    let (public_key, n, private_key, _) = key_gen();

    // Message to sign
    let message = b"Hello, World!";

    // Hash the message
    let message_hash = hash_message(message);

    // Sign the hash
    let signature = sign(&message_hash, &private_key, &n);

    // Verify the signature
    let is_valid = verify(&signature, &public_key, &n, &message_hash);

    if is_valid {
        println!("Signature is valid!");
    } else {
        println!("Signature is not valid.");
    }
}