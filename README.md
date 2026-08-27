# orx-parallel-benchmarks

Benchmarks for parallel computations, with a primary focus on the
[`orx-parallel`](https://github.com/orxfun/orx-parallel) crate.

## Why a separate benchmark runner?

We mostly  use `criterion`, often together with `orx-criterion`.
But using `criterion` with multiple thread pools alive
at once can affect the results. Each benchmark is therefore run with a single
alive thread pool. The process ends before another pool is started for the
next variant. The emphasis is on measurement accuracy.

The runner sets both `RAYON_NUM_THREADS` and `ORX_NUM_THREADS` for each run.

Before every benchmark run, it executes `cargo update` so the dependency
version selected by the current branch is used.

## Layout

Benchmark sources are organized as:

```text
<category>/<benchmark>/
```

For example, `collect/filter` is the `filter` benchmark in the `collect`
category. Each benchmark is a standalone Cargo project, and its Cargo
features select the implementations to compare.

## Run one benchmark directly

You may run each benchmark project alone providing a feature which corresponds to the computation variant.

```sh
# go into the benchmark project directory
cd recursive/tree_collect

# to run the sequential variant
RAYON_NUM_THREADS=16 ORX_NUM_THREADS=16 \
cargo run --release --features seq -- --run-mode run --warmup-runs 2 --actual-runs 10

# to run the rayon variant
RAYON_NUM_THREADS=16 ORX_NUM_THREADS=16 \
cargo run --release --features rayon -- --run-mode run --warmup-runs 2 --actual-runs 10

# to run the orx-parallel variant using basic (default) thread pool
RAYON_NUM_THREADS=16 ORX_NUM_THREADS=16 \
cargo run --release --features orx-basic -- --run-mode run --warmup-runs 2 --actual-runs 10

# to run the orx-parallel variant using once thread pool
RAYON_NUM_THREADS=16 ORX_NUM_THREADS=16 \
cargo run --release --features orx-once -- --run-mode run --warmup-runs 2 --actual-runs 10
```


## Run one benchmark with `bench-runner`

From the repository root, run `bench-runner` with the benchmark path and an
output CSV path:

```sh
cargo run --release --manifest-path bench-runner/Cargo.toml -- \
	--path collect/filter \
	--path-result benchmarks-ui/results/collect/filter.csv \
	--warmup-runs 20 \
	--actual-runs 100 \
	--threads 4 --threads 8 --threads 16
```

## Run all benchmarks

`bench-runner-all/run-all.sh` runs the configured categories and thread counts:

```sh
cd bench-runner-all
./run-all.sh
```

The outputs are written under `benchmarks-ui/results/`. You can also invoke the
runner directly to choose categories or thread counts:

```sh
cargo run --release --manifest-path bench-runner-all/Cargo.toml -- \
	--path . \
	--path-result benchmarks-ui/results \
	--warmup-runs 20 --actual-runs 100 \
	--threads 4 --threads 8 \
	--categories collect
```

## Testing branches

This repository is useful for testing new `orx-parallel` branches. Change the
`branch` in the benchmark Cargo manifests, then run a benchmark; its
pre-run `cargo update` ensures that the selected branch dependency is used.
