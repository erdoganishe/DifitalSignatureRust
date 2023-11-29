pub mod rsa_digital_signature {

    use num_bigint::{BigUint, ToBigUint};
    use rand::Rng;
    use num_integer::{Integer};
    use num_traits::{One, Zero, ToPrimitive};


    pub fn key_gen() -> (BigUint, BigUint, BigUint, BigUint) {
        let mut rng = rand::thread_rng();

        // Generate two large prime numbers P and Q
        let p = generate_large_prime();
        let q = generate_large_prime();

        // Calculate n = P * Q
        let n = &p * &q;

        // Calculate Euler's totient function φ(n) = (P-1)(Q-1)
        let phi_n = (p - 1u32).to_biguint().unwrap() * (q - 1u32).to_biguint().unwrap();

        // Choose public exponent e, where 1 < e < φ(n) and e is coprime to φ(n)
        let e = find_public_exponent(&phi_n);

        // Calculate private exponent d such that (d * e) % φ(n) = 1
        let d = mod_inverse(&e, &phi_n).unwrap();

        (e, n.clone(), d, n)
    }

    pub fn find_public_exponent(phi_n: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();
    
        // Starting with a random number between 2 and phi_n - 1
        let mut e = rng.gen::<u32>().to_biguint().unwrap();

        while (&e < phi_n) {
            e = rng.gen::<u32>().to_biguint().unwrap();
        }

    
        // Keep trying different values until we find one that is coprime with phi_n
        while e.gcd(phi_n) != BigUint::one() {
            e += BigUint::one();
        }
    
        e
    }

    pub fn generate_large_prime() -> BigUint {
        let mut rng = rand::thread_rng();
    
        loop {

            let candidate = rng.gen::<u32>().to_biguint().unwrap();

            if is_prime(&candidate) {
                return candidate;
            }
        }
    }

    pub fn is_prime(n: &BigUint) -> bool {
        if n <= &1u32.to_biguint().unwrap() {
            return false;
        }
    
        if n == &2u32.to_biguint().unwrap() || n == &3u32.to_biguint().unwrap() {
            return true;
        }

        let sqrt_n = n.sqrt().to_u32().unwrap();
        for i in (2..=sqrt_n).map(|x| x.to_biguint().unwrap()) {
            if n % &i == BigUint::zero() {
                return false;
            }
        }
    
        true

    }

    fn extended_gcd(a: &BigUint, b: &BigUint) -> (BigUint, BigUint, BigUint) {
        if a.is_zero() {
            return (b.clone(), BigUint::zero(), BigUint::one());
        }
    
        let (g, x, y) = extended_gcd(&(b % a), a);
    
        let new_x = &y - &(b / a) * &x;
        let new_y = x;
    
        (g, new_x, new_y)
    }
    
    pub fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
        let (g, x, _) = extended_gcd(a, m);
    
        if g == BigUint::one() {
            Some((x % m + m) % m)
        } else {
            None  // The modular inverse doesn't exist if gcd(a, m) != 1
        }
    }


}


