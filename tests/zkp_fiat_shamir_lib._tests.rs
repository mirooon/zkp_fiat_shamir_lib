use zkp_fiat_shamir_lib::*;

#[test]
fn test_nizk() {
    // public parameters
    let g = 5;
    let p = 23;

    // prover's secret
    let x = 7;

    // prover generates commitment
    let r = 33; // randomly chosen
    let t = generate_commitment(g, r, p);  // t = g^r % p

    // compute public value y = g^x % p
    let y = mod_exp(g, x, p);

    // verifier generates challenge using Fiat-Shamir heuristic
    let c = generate_challenge(t, y, g, p);

    // prover generates response
    let s = generate_response(r, c, x, p);

    // verifier checks the proof
    assert!(verify_proof(t, y, g, s, c, p));
}
