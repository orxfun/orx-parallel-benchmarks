use crate::alg::Method;
use crate::input::{ComputeType, InputVariant, PipelineType};
use bench_helper::runner::cpu_mix;
use orx_criterion::Experiment;
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::hint::black_box;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Agg {
    pub sum: u64,
    pub xor_sum: u64,
    pub count: u64,
}

impl Agg {
    #[inline(always)]
    pub fn from_val(v: u64) -> Self {
        Self {
            sum: v,
            xor_sum: v,
            count: 1,
        }
    }
}

#[inline(always)]
pub fn merge(a: Agg, b: Agg) -> Agg {
    Agg {
        sum: a.sum.wrapping_add(b.sum),
        xor_sum: a.xor_sum ^ b.xor_sum,
        count: a.count + b.count,
    }
}

// ---------------------------------------------------------------------------
// Compute functions per item
// ---------------------------------------------------------------------------

#[inline(always)]
fn compute_light(x: u64) -> u64 {
    let x = black_box(x);
    x.rotate_left(13) ^ 0x5555_AAAA_3333_CCCC ^ (x.wrapping_mul(0x9E37_79B9_7F4A_7C15))
}

#[inline(always)]
fn compute_medium(x: u64) -> u64 {
    cpu_mix(50, black_box(x ^ 0xDEAD_BEEF_CAFE_0123))
}

#[inline(always)]
fn compute_heavy(x: u64) -> u64 {
    cpu_mix(500, black_box(x ^ 0xFEED_FACE_CAFE_BABE))
}

#[inline(always)]
fn compute_variable(x: u64) -> u64 {
    if x % 16 == 0 {
        cpu_mix(500, black_box(x))
    } else {
        compute_light(x)
    }
}

#[inline(always)]
fn apply_compute(compute_type: ComputeType, val: u64) -> u64 {
    match compute_type {
        ComputeType::Light => compute_light(val),
        ComputeType::Medium => compute_medium(val),
        ComputeType::Heavy => compute_heavy(val),
        ComputeType::Variable => compute_variable(val),
    }
}

// ---------------------------------------------------------------------------
// Experiment Implementation
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub struct InputData {
    data: Vec<u64>,
}

pub struct Exp;

impl Experiment for Exp {
    type InputFactors = InputVariant;
    type AlgFactors = Method;
    type Input = InputData;
    type Output = Agg;

    fn input(&mut self, input_variant: &Self::InputFactors) -> Self::Input {
        const SEED: u64 = 0x5426_1234_BEEF_CAFE;
        let mut rng = ChaCha8Rng::seed_from_u64(SEED ^ (input_variant.n as u64));
        let data: Vec<u64> = (0..input_variant.n)
            .map(|_| rng.random_range(1..=1_000_000))
            .collect();
        InputData { data }
    }

    fn execute(
        &mut self,
        input_variant: &Self::InputFactors,
        alg_variant: &Self::AlgFactors,
        input: &Self::Input,
    ) -> Self::Output {
        let compute = input_variant.compute;
        let pipeline = input_variant.pipeline;
        let data = &input.data;

        match alg_variant {
            Method::Seq => execute_seq(data, pipeline, compute),
            Method::Rayon => execute_rayon(data, pipeline, compute),
            Method::OrxOnce | Method::OrxBasic | Method::OrxRayon => {
                execute_orx(data, pipeline, compute)
            }
        }
    }

    fn expected_output(
        &self,
        input_variant: &Self::InputFactors,
        input: &Self::Input,
    ) -> Option<Self::Output> {
        Some(execute_seq(
            &input.data,
            input_variant.pipeline,
            input_variant.compute,
        ))
    }
}

// ---------------------------------------------------------------------------
// Execution Strategies
// ---------------------------------------------------------------------------

fn execute_seq(data: &[u64], pipeline: PipelineType, compute: ComputeType) -> Agg {
    match pipeline {
        PipelineType::FilterMap => data
            .iter()
            .copied()
            .filter(|&x| x % 2 == 0)
            .map(|x| apply_compute(compute, x))
            .map(Agg::from_val)
            .fold(Agg::default(), merge),
        PipelineType::Map => data
            .iter()
            .copied()
            .map(|x| x.wrapping_add(1))
            .map(|x| apply_compute(compute, x))
            .map(Agg::from_val)
            .fold(Agg::default(), merge),
    }
}

fn execute_rayon(data: &[u64], pipeline: PipelineType, compute: ComputeType) -> Agg {
    use rayon::prelude::*;
    match pipeline {
        PipelineType::FilterMap => data
            .iter()
            .copied()
            .filter(|&x| x % 2 == 0)
            .par_bridge()
            .map(|x| apply_compute(compute, x))
            .map(Agg::from_val)
            .reduce(Agg::default, merge),
        PipelineType::Map => data
            .iter()
            .copied()
            .map(|x| x.wrapping_add(1))
            .par_bridge()
            .map(|x| apply_compute(compute, x))
            .map(Agg::from_val)
            .reduce(Agg::default, merge),
    }
}

fn execute_orx(data: &[u64], pipeline: PipelineType, compute: ComputeType) -> Agg {
    use orx_parallel::*;
    match pipeline {
        PipelineType::FilterMap => data
            .iter()
            .copied()
            .filter(|&x| x % 2 == 0)
            .iter_into_par()
            .map(|x| apply_compute(compute, x))
            .map(Agg::from_val)
            .reduce(merge)
            .unwrap_or_default(),
        PipelineType::Map => data
            .iter()
            .copied()
            .map(|x| x.wrapping_add(1))
            .iter_into_par()
            .map(|x| apply_compute(compute, x))
            .map(Agg::from_val)
            .reduce(merge)
            .unwrap_or_default(),
    }
}
