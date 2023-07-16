//! Prime field `Fp` where `p = 9 mod 16`.
use ark_ff::fields::{Fp128, MontBackend};

#[derive(ark_ff::MontConfig)]
#[modulus = "294894660409212688958748647155473271321"]
#[generator = "31"]
pub struct FqConfig;
pub type Fq = Fp128<MontBackend<FqConfig, 2>>;

#[cfg(test)]
mod tests {
    use super::*;
    use ark_algebra_test_templates::*;
    test_field!(fq; Fq; mont_prime_field);
}
