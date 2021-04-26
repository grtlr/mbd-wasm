# mbd-wasm

[![GitHub Actions](https://github.com/grtlr/mbd-wasm/actions/workflows/ci.yml/badge.svg)](https://github.com/grtlr/mbd-wasm/actions/workflows/ci.yml)
[![npm version](https://img.shields.io/npm/v/mbd-wasm.svg)](https://www.npmjs.com/package/mbd-wasm)
[![Crates.io](https://img.shields.io/crates/v/mbd.svg)](https://crates.io/crates/mbd)

A Rust implementation of the modified band depth that also compiles to WASM.

## Usage

The library can be used a simple rust crate, by adding `mbd = "*"` to your `Cargo.toml`. Then, we can compute the modified band depth of the functional `[2.0, 3.0, 4.0]` as follows:

```rust
let data = vec![vec![4.0, 5.0, 6.0], vec![1.0, 2.0, 3.0]];
let mbd = ModifiedBandDepth::from_samples(&data);
assert_eq!(mbd.query(&[2.0, 3.0, 4.0]), 1.0);
```

Similarly, you can call the the same functionality from JavaScript:

```js
mbdWasm = (await require('mbd-wasm@0.0.4'))();
const data = [[4.0, 5.0, 6.0], [1.0, 2.0, 3.0]]
const num_samples = data.length;
const num_timepoints = data[0].length;
const mbd = mbdWasm.ModifiedBandDepth.from_data_matrix(num_samples, num_timepoints, data.flat());
console.log(mbd.query([2.0, 3.0, 4.0])); // prints 1.0
```
