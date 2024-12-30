use bellperson::groth16::{create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof};
use pairing::bls12_381::Bls12;
use std::time::Instant;
use crate::circuit::large_circuit::LargeCircuit;

pub fn generate_and_verify_proof(circuit: LargeCircuit) {
    // 生成 proving parameters
    println!("Generating proving parameters...");
    let params = {
        let empty_circuit = LargeCircuit::new(0); // 空电路用于初始化参数
        generate_random_parameters::<Bls12, _, _>(empty_circuit, &mut rand::thread_rng()).unwrap()
    };

    // 生成 proof
    println!("Creating proof...");
    let start = Instant::now();
    let proof = create_random_proof(circuit, &params, &mut rand::thread_rng()).unwrap();
    let elapsed = start.elapsed();
    println!("Proof generation completed in {:.2?}", elapsed);

    // 准备验证
    println!("Preparing verification...");
    let pvk = prepare_verifying_key(&params.vk);

    // 验证证明
    let is_valid = verify_proof(&pvk, &proof, &[Bls12::Fr::one()]).unwrap();
    println!("Proof is valid: {}", is_valid);
}
