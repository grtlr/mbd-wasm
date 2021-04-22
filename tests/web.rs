//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

// For now we test with Node only.
// wasm_bindgen_test_configure!(run_in_browser);

extern crate mbd;

#[wasm_bindgen_test]
fn test_fully_contained() {
    let rows = 2;
    let timepoints = 3;
    let data = [4.0, 5.0, 6.0, 1.0, 2.0, 3.0];
    let mbd = mbd::ModifiedBandDepth::from_data_matrix(rows, timepoints, &data);
    assert_eq!(mbd.query(&[2.0, 3.0, 4.0]), 1.0);
}