mod alg;
mod exp;
mod input;

use crate::{
    alg::Method,
    exp::Exp,
    input::{ElementType, InputVariant},
};
use bench_helper::{BenchArgs, runner};
use clap::Parser;

fn main() {
    let args = BenchArgs::parse();

    let ns = [16, 20];
    let element_types = [ElementType::U64, ElementType::String];

    let combine_types = |n| {
        element_types.map(|element_type| InputVariant { n, element_type })
    };
    let input_variants: Vec<_> = ns.into_iter().flat_map(combine_types).collect();

    runner::run(&args, Exp, &input_variants, &Method::get());
}
