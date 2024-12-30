mod circuit;
mod config;

use circuit::large_circuit::LargeCircuit;
use circuit::utils::generate_random_inputs;
use bellperson::groth16::{create_random_proof, generate_random_parameters, prepare_verifying_key, verify_proof};
use blstrs::Bls12;
use std::time::Instant;

fn main() {
    // 配置电路大小
    let size = 1 << 20; // 电路输入大小
    println!("Generating large circuit with {} inputs...", size);

    // 创建电路
    let inputs = generate_random_inputs(size);
    let circuit = LargeCircuit { inputs };

    // 生成证明参数
    println!("Generating proving parameters...");
    let params = {
        let empty_circuit = LargeCircuit { inputs: vec![None; size] };
        generate_random_parameters::<Bls12, _, _>(empty_circuit, &mut rand::thread_rng()).unwrap()
    };

    // 生成证明
    println!("Creating proof...");
    let start = Instant::now();
    let proof = create_random_proof(circuit, &params, &mut rand::thread_rng()).unwrap();
    let elapsed = start.elapsed();
    println!("Proof generation completed in {:.2?}", elapsed);

    // 准备验证
    let pvk = prepare_verifying_key(&params.vk);

    // 验证证明
    println!("Verifying proof...");
    let is_valid = verify_proof(&pvk, &proof, &[Bls12::Fr::one()]).unwrap();
    println!("Proof is valid: {}", is_valid);
}
