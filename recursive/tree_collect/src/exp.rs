use crate::{WEIGHT, alg::Method, input::InputVariant};
use bench_helper::runner::cpu_mix;
use orx_criterion::Experiment;
use orx_parallel::*;
use rayon::prelude::*;

pub struct Exp;

pub struct Node {
    value: u64,
    children: Vec<Node>,
}

fn build_tree(depth: usize, fan_out: usize, seed: u64) -> Node {
    let children = if depth == 0 {
        vec![]
    } else {
        (0..fan_out)
            .map(|index| build_tree(depth - 1, fan_out, seed ^ index as u64))
            .collect()
    };
    Node {
        value: cpu_mix(2, seed),
        children,
    }
}

fn matches(node: &Node, threshold: u64) -> bool {
    let node_value = cpu_mix(WEIGHT, node.value);
    node_value % 10_000 < threshold
}

fn collect_seq<'a>(node: &'a Node, threshold: u64, nodes: &mut Vec<&'a Node>) {
    if matches(node, threshold) {
        nodes.push(node);
    }
    for child in &node.children {
        collect_seq(child, threshold, nodes);
    }
}

fn collect_rayon(node: &Node, threshold: u64) -> Vec<&Node> {
    let mut nodes = match matches(node, threshold) {
        true => vec![node],
        false => vec![],
    };

    nodes.extend(
        node.children
            .par_iter()
            .map(|child| collect_rayon(child, threshold))
            .reduce(Vec::new, |mut left, mut right| {
                left.append(&mut right);
                left
            }),
    );
    nodes
}

fn collect_orx(node: &Node, threshold: u64) -> Vec<&Node> {
    par_recursive([node], |node| &node.children)
        .filter(|node| matches(node, threshold))
        .collect()
}

impl Experiment for Exp {
    type InputFactors = InputVariant;
    type AlgFactors = Method;
    type Input = Node;
    type Output = usize;

    fn input(&mut self, input_variant: &Self::InputFactors) -> Self::Input {
        build_tree(input_variant.depth, input_variant.fan_out, 42)
    }

    fn execute(
        &mut self,
        input_variant: &Self::InputFactors,
        method: &Self::AlgFactors,
        input: &Self::Input,
    ) -> Self::Output {
        match method {
            Method::Seq => {
                let mut nodes = Vec::new();
                collect_seq(input, input_variant.threshold, &mut nodes);
                nodes.len()
            }
            Method::Rayon => collect_rayon(input, input_variant.threshold).len(),
            Method::OrxOnce | Method::OrxBasic | Method::OrxRayon => {
                collect_orx(input, input_variant.threshold).len()
            }
        }
    }

    fn expected_output(
        &self,
        input_variant: &Self::InputFactors,
        input: &Self::Input,
    ) -> Option<Self::Output> {
        let mut nodes = Vec::new();
        collect_seq(input, input_variant.threshold, &mut nodes);
        Some(nodes.len())
    }
}
