use crate::{alg::Method, input::InputVariant};
use bench_helper::runner;
use orx_criterion::Experiment;
use orx_parallel::IterationOrder;
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::hint::black_box;

const FIB_UPPER_BOUND: u64 = 99;

pub struct Exp;

impl Experiment for Exp {
    type InputFactors = InputVariant;

    type AlgFactors = Method;

    type Input = Vec<u64>;

    type Output = (bool, Output); // (ordered, output)

    fn input(&mut self, input_variant: &Self::InputFactors) -> Self::Input {
        const SEED: u64 = 654;
        let len = input_variant.len();
        let mut rng = ChaCha8Rng::seed_from_u64(SEED);
        (0..len).map(|_| rng.random_range(0..150)).collect()
    }

    fn execute(
        &mut self,
        input_variant: &Self::InputFactors,
        alg_variant: &Self::AlgFactors,
        input: &Self::Input,
    ) -> Self::Output {
        let h = input_variant.heavy;
        match alg_variant {
            Method::Seq => (true, run_seq(input, h)),
            Method::Rayon => (true, run_rayon(input, h)),
            Method::OrxOnce => (true, run_orx(input, h, IterationOrder::Ordered)),
            Method::OrxBasic => (true, run_orx(input, h, IterationOrder::Ordered)),
            Method::OrxRayon => (true, run_orx(input, h, IterationOrder::Ordered)),
        }
    }

    fn validate_output(
        &self,
        input_variant: &Self::InputFactors,
        input: &Self::Input,
        (ordered, output): &Self::Output,
    ) {
        let mut expected = match run_seq(input, input_variant.heavy) {
            Output::Vec(vec) => vec,
        };

        if !*ordered {
            expected.sort();
        }

        match output {
            Output::Vec(result) => match *ordered {
                false => {
                    let mut result = result.clone();
                    result.sort();
                    assert_eq!(expected, result)
                }
                true => assert_eq!(&expected, result),
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Output {
    Vec(Vec<u64>),
}

fn l_m(x: &u64) -> impl IntoIterator<Item = u64> {
    (0..7).map(|y| match *x + y {
        999 => 999,
        n => 7 * n + 1000,
    })
}

fn h_m(x: &u64) -> impl IntoIterator<Item = u64> {
    (0..7).map(|y| {
        let f = black_box(runner::fib(FIB_UPPER_BOUND, *x + y));
        let g = black_box(*x + f);
        match f + g {
            999 => g - f,
            n => 7 * n + 1000,
        }
    })
}

fn run_seq(input: &[u64], heavy: bool) -> Output {
    match heavy {
        true => Output::Vec(input.iter().flat_map(h_m).collect()),
        false => Output::Vec(input.iter().flat_map(l_m).collect()),
    }
}

fn run_rayon(input: &[u64], heavy: bool) -> Output {
    use rayon::prelude::*;
    match heavy {
        true => Output::Vec(input.into_par_iter().flat_map_iter(h_m).collect()),
        false => Output::Vec(input.into_par_iter().flat_map_iter(l_m).collect()),
    }
}

fn run_orx(input: &[u64], heavy: bool, ord: IterationOrder) -> Output {
    use orx_parallel::*;
    let par = input.into_par().iteration_order(ord);
    match heavy {
        true => Output::Vec(par.flat_map(h_m).collect()),
        false => Output::Vec(par.flat_map(l_m).collect()),
    }
}
