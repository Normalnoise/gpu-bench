use bellperson::groth16::{create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof};
use blstrs::{Bls12, Scalar as Fr};
use ff::PrimeField; // 引入 PrimeField
use rand::thread_rng;
use std::time::Instant;

fn main() {
    let size = 1 << 10; // 电路输入大小
    let inputs = vec![None; size];
    let circuit = LargeCircuit { inputs };

    let params = {
        let empty_circuit = LargeCircuit { inputs: vec![None; size] };
        generate_random_parameters::<Bls12, _, _>(empty_circuit, &mut thread_rng()).unwrap()
    };

    println!("Creating proof...");
    let start = Instant::now();
    let proof = create_random_proof(circuit, &params, &mut thread_rng()).unwrap();
    let elapsed = start.elapsed();
    println!("Proof generation completed in {:.2?}", elapsed);

    let pvk = prepare_verifying_key(&params.vk);

    let is_valid = verify_proof(&pvk, &proof, &[<Bls12 as bellperson::Engine>::Fr::one()]).unwrap();
    println!("Proof is valid: {}", is_valid);
}
