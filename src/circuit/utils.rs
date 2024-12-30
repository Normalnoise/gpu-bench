use blstrs::Scalar as Fr;
use ff::Field; // 引入 Field trait
use rand::thread_rng;

/// 生成指定大小的随机输入
pub fn generate_random_inputs(size: usize) -> Vec<Option<Fr>> {
    let mut rng = thread_rng();
    (0..size).map(|_| Some(Fr::random(&mut rng))).collect()
}
