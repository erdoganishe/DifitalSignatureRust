pub mod rsa_digital_signature {

    use num_bigint::{BigUint, ToBigUint};
    use rand::Rng;
    use num_integer::Integer;
    use num_traits::{One, Zero, ToPrimitive};


    pub fn key_gen() -> (BigUint, BigUint, BigUint, BigUint) {
        let mut rng = rand::thread_rng();
    
        // Generate two large prime numbers P and Q
        let p = generate_large_prime();
        let q = generate_large_prime();
    
        println!("Generated primes: P = {}, Q = {}", &p, &q);
    
        // Ensure (p - 1) and (q - 1) are greater than or equal to 2
        let phi_p = if p > 2u32.to_biguint().unwrap() { p.clone() - 1u32 } else { 1u32.to_biguint().unwrap() };
        let phi_q = if q > 2u32.to_biguint().unwrap() { q.clone() - 1u32 } else { 1u32.to_biguint().unwrap() };
    
        // Calculate n = P * Q
        let n = &p * &q;
    
        // Calculate Euler's totient function φ(n) = (P-1)(Q-1)
        let phi_n = phi_p * phi_q;
    
        println!("Euler's totient function: φ(n) = {}", &phi_n);
    
        // Choose public exponent e, where 1 < e < φ(n) and e is coprime to φ(n)
        let e = find_public_exponent(&phi_n);
    
        println!("Public exponent: e = {}", &e);
    
        // Calculate private exponent d such that (d * e) % φ(n) = 1
        let d = mod_inverse(&e, &phi_n).unwrap();
    
        println!("Private exponent: d = {}", &d);
    
        (e, n.clone(), d, n)
    }
    
    

    pub fn find_public_exponent(phi_n: &BigUint) -> BigUint {
        let mut rng = rand::thread_rng();
    
        // Generate a random number between 2 and phi_n - 1
        let range = (BigUint::from(2u32), phi_n.clone() - BigUint::one());

        // Convert the range to a u64 range
        let u64_range = (range.0.to_u64().unwrap(), range.1.to_u64().unwrap());

        // Generate a random u64
        let random_u64 = rng.gen_range(u64_range.0..=u64_range.1);

        // Convert the u64 back to BigUint
        let mut e = random_u64.to_biguint().unwrap();

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



    pub fn mod_inverse(a: &BigUint, m: &BigUint) -> Option<BigUint> {
        let (g, x, _) = extended_gcd(a, m);
    
        if g == BigUint::one() {
            Some(x % m)
        } else {
            None 
        }
    }
    
    fn extended_gcd(a: &BigUint, b: &BigUint) -> (BigUint, BigUint, BigUint) {
        if a.is_zero() {
            (b.clone(), BigUint::zero(), BigUint::one())
        } else {
            let (g, x, y) = extended_gcd(&(b % a), a);
            print!("{} - {}", y.clone(), &(b / a) * &x);
            let x1 = y.clone() - &(b / a) * &x;
            (g, x1, x)
        }
    }
    

}


