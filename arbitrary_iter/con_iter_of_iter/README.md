# ConIterOfIter (Generic / Arbitrary Iterator) Benchmark

Benchmarks parallel processing over arbitrary/generic non-slice iterators using `orx-parallel` (which uses `ConIterOfIter` underneath), compared against sequential iteration and Rayon's `par_bridge`.

## Methods

- `seq`: Sequential baseline (`Iterator::map / filter / fold`).
- `rayon`: Rayon's `ParallelBridge` (`par_bridge()`).
- `orx-once`: One-shot thread pool (`iter_into_par()`).
- `orx-basic`: Persistent thread pool (`iter_into_par()` with `orx-parallel/persistent-pool`).
- `orx-rayon`: Rayon-backed thread pool (`iter_into_par()` with `orx-parallel/persistent-pool-rayon`).

## Input Factors

- **Size (`n`)**: Sequence length (`16_384`, `65_536`).
- **Compute Type (`compute`)**:
  - `light`: Low-cost arithmetic/bitwise operation (reveals synchronization overhead).
  - `medium`: Moderate CPU workload (`cpu_mix(50)`).
  - `heavy`: Heavy CPU workload (`cpu_mix(500)`).
  - `variable`: Skewed workload (10% heavy, 90% light) testing dynamic load balancing.
  - `alloc`: Heap allocations with nested structures (`String` and nested `Vec`s).
- **Pipeline Type (`pipeline`)**:
  - `filter_map`: `filter` + `map` pipeline filtering ~50% of items.
  - `map`: `map` pipeline transforming 100% of items.

## How to Run

```bash
# Run with bench-runner
cargo run --release --manifest-path ../../bench-runner/Cargo.toml -- \
    --path . \
    --path-result results.csv \
    --warmup-runs 5 \
    --actual-runs 20 \
    --threads 4 \
    --threads 8 \
    --threads 16
```
