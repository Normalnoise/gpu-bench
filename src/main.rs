mod circuit;
mod proof;
mod config;

use circuit::large_circuit::LargeCircuit;
use config::calculate_max_input_size;
use proof::proof_generation::generate_and_verify_proof;

fn main() {
    // 动态调整输入大小
    let input_size = calculate_max_input_size();
    println!("Generating large circuit with {} inputs...", input_size);

    // 生成随机电路
    let circuit = LargeCircuit::new(input_size);

    // 生成和验证 proof
    generate_and_verify_proof(circuit);
}
