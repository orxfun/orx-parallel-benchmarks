use crate::alg::Method;
use crate::input::{ElementType, InputVariant};
use orx_criterion::Experiment;
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::cell::UnsafeCell;

pub struct Exp;

pub enum Input {
    U64(UnsafeCell<Vec<u64>>),
    String(UnsafeCell<Vec<String>>),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Output {
    U64(Option<u64>),
    String(Option<String>),
}

impl Experiment for Exp {
    type InputFactors = InputVariant;

    type AlgFactors = Method;

    type Input = Input;

    type Output = Output;

    fn input(&mut self, input_variant: &Self::InputFactors) -> Self::Input {
        const SEED: u64 = 654;
        let len = input_variant.len();
        let mut rng = ChaCha8Rng::seed_from_u64(SEED);
        match input_variant.element_type {
            ElementType::U64 => Input::U64(UnsafeCell::new(
                (0..len).map(|_| rng.random_range(0..u64::MAX)).collect(),
            )),
            ElementType::String => Input::String(UnsafeCell::new(
                (0..len)
                    .map(|_| format!("key_{:016x}", rng.random_range(0..u64::MAX)))
                    .collect(),
            )),
        }
    }

    fn execute(
        &mut self,
        _input_variant: &Self::InputFactors,
        alg_variant: &Self::AlgFactors,
        input: &Self::Input,
    ) -> Self::Output {
        match alg_variant {
            Method::Seq => run_seq(input),
            Method::Rayon => run_rayon(input),
            Method::OrxOnce => run_orx(input),
            Method::OrxBasic => run_orx(input),
            Method::OrxRayon => run_orx(input),
        }
    }

    fn validate_output(
        &self,
        _input_variant: &Self::InputFactors,
        input: &Self::Input,
        output: &Self::Output,
    ) {
        let expected = run_seq(input);
        assert_eq!(&expected, output);
    }
}

fn run_seq(input: &Input) -> Output {
    match input {
        Input::U64(vec) => {
            let vec = unsafe { &mut *vec.get() };
            vec.sort_unstable();
            Output::U64(vec.last().cloned())
        }
        Input::String(vec) => {
            let vec = unsafe { &mut *vec.get() };
            vec.sort_unstable();
            Output::String(vec.last().cloned())
        }
    }
}

fn run_rayon(input: &Input) -> Output {
    use rayon::slice::ParallelSliceMut;
    match input {
        Input::U64(vec) => {
            let vec = unsafe { &mut *vec.get() };
            vec.par_sort_unstable();
            Output::U64(vec.last().cloned())
        }
        Input::String(vec) => {
            let vec = unsafe { &mut *vec.get() };
            vec.par_sort_unstable();
            Output::String(vec.last().cloned())
        }
    }
}

fn run_orx(input: &Input) -> Output {
    use orx_parallel::*;
    match input {
        Input::U64(vec) => {
            let vec = unsafe { &mut *vec.get() };
            let mut runner = Runner::fixed();
            par_experimental_sort(vec, &mut runner, Params::default());
            Output::U64(vec.last().cloned())
        }
        Input::String(vec) => {
            let vec = unsafe { &mut *vec.get() };
            let mut runner = Runner::fixed();
            par_experimental_sort(vec, &mut runner, Params::default());
            Output::String(vec.last().cloned())
        }
    }
}
