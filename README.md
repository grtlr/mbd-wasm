# mbd-wasm

A Rust implementation of the modified band depth that also compiles to WASM.

## Usage

Using from Rust:

```rust
let data = vec![vec![4.0, 5.0, 6.0], vec![1.0, 2.0, 3.0]];
let mbd = ModifiedBandDepth::from_samples(&data);
assert_eq!(mbd.query(&[2.0, 3.0, 4.0]), 1.0);
```

## Publish

```sh
yarn install
yarn run build
```
