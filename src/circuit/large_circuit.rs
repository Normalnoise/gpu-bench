use bellperson::{Circuit, ConstraintSystem, SynthesisError};
use blstrs::Scalar as Fr;
use rand::thread_rng;

pub struct LargeCircuit {
    pub inputs: Vec<Option<Fr>>,
}

impl LargeCircuit {
    /// 创建具有随机输入的电路
    pub fn new(size: usize) -> Self {
        let inputs = (0..size).map(|_| Some(Fr::random(thread_rng()))).collect();
        LargeCircuit { inputs }
    }
}

impl Circuit<Fr> for LargeCircuit {
    fn synthesize<CS: ConstraintSystem<Fr>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let mut vars = vec![];

        // 分配输入变量
        for (i, input) in self.inputs.iter().enumerate() {
            let var = cs.alloc(
                || format!("input_{}", i),
                || input.ok_or(SynthesisError::AssignmentMissing),
            )?;
            vars.push(var);
        }

        // 构造约束
        let mut current = vars[0];
        for i in 1..vars.len() {
            let next = cs.alloc(
                || format!("intermediate_{}", i),
                || {
                    let a = self.inputs[i - 1].ok_or(SynthesisError::AssignmentMissing)?;
                    let b = self.inputs[i].ok_or(SynthesisError::AssignmentMissing)?;
                    Ok(a * b + a.square())
                },
            )?;
            cs.enforce(
                || format!("constraint_{}", i),
                |lc| lc + current,
                |lc| lc + vars[i],
                |lc| lc + next,
            );
            current = next;
        }

        // 分配输出
        cs.alloc_input(
            || "output",
            || {
                let result = self.inputs.iter().fold(Fr::one(), |acc, &val| {
                    let val = val.unwrap_or(Fr::zero());
                    acc * val.square()
                });
                Ok(result)
            },
        )?;

        Ok(())
    }
}
