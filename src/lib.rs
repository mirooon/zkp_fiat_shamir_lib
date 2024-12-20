use sha2::{Digest, Sha256};

/// modular exponentiation: computes base^exp % modulus
pub fn mod_exp(base: u64, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exp;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        exp /= 2;
        base = (base * base) % modulus;
    }
    result
}

/// commitment generation: computes t = g^r % p
pub fn generate_commitment(g: u64, r: u64, p: u64) -> u64 {
    mod_exp(g, r, p)
}

/// challenge generation using Fiat-Shamir heuristic
pub fn generate_challenge(t: u64, y: u64, g: u64, p: u64) -> u64 {
    // concatenate inputs as a single string
    let input = format!("{},{},{},{}", t, y, g, p);
    let hash = Sha256::digest(input.as_bytes());
    // convert the first 8 bytes of the hash to a u64 and mod it with p-1
    let challenge = u64::from_be_bytes(hash[0..8].try_into().unwrap()) % (p - 1);
    challenge
}

/// response generation: computes s = (r + c * x) % (p-1)
pub fn generate_response(r: u64, c: u64, x: u64, p: u64) -> u64 {
    (r + c * x) % (p - 1)
}

/// verifies the proof: checks if g^s % p == t * y^c % p
pub fn verify_proof(t: u64, y: u64, g: u64, s: u64, c: u64, p: u64) -> bool {
    let left = mod_exp(g, s, p); // g^s % p
    let right = (t * mod_exp(y, c, p)) % p; // t * y^c % p
    left == right
}
