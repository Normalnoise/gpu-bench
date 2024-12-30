use bellperson::{Circuit, ConstraintSystem, SynthesisError};
use blstrs::Scalar as Fr;
use rand::thread_rng;

pub struct LargeCircuit {
    pub inputs: Vec<Option<Fr>>,
}

impl LargeCircuit {
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

        for (i, input) in self.inputs.iter().enumerate() {
            let var = cs.alloc(
                || format!("input_{}", i),
                || input.ok_or(SynthesisError::AssignmentMissing),
            )?;
            vars.push(var);
        }

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
