use pairing::bls12_381::Fr;
use rand::thread_rng;

pub fn generate_random_inputs(size: usize) -> Vec<Option<Fr>> {
    let mut rng = thread_rng();
    (0..size).map(|_| Some(Fr::random(&mut rng))).collect()
}
