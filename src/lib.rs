mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[derive(Debug, PartialEq)]
pub struct ModifiedBandDepth {
    num_samples: usize,
    num_timepoints: usize,
    sorted_matrix: Vec<f32>, // column major
}

#[wasm_bindgen]
impl ModifiedBandDepth {
    pub fn from_data_matrix(rows: usize, timepoints: usize, data_matrix: &[f32]) -> Self {
        let mut sorted_matrix = Vec::with_capacity(rows * timepoints);
        for j in 0..timepoints {
            for i in 0..rows {
                sorted_matrix.push(data_matrix[i * timepoints + j]);
            }
            sorted_matrix
                .chunks_mut(rows)
                .for_each(|c| c.sort_by(|a, b| a.partial_cmp(b).unwrap()));
        }
        Self {
            num_samples: rows,
            num_timepoints: timepoints,
            sorted_matrix,
        }
    }

    pub fn query(&self, sample: &[f32]) -> f32 {
        debug_assert_eq!(sample.len(), self.num_timepoints);
        let mut count = 0;
        for (sorted_ys, &v) in self
            .sorted_matrix
            .chunks(self.num_samples)
            .zip(sample.iter())
        {
            let (lt, eq, gt) = partition(sorted_ys, v);
            count += lt * eq + eq * gt + lt * gt + binomial_choose_2(eq);
        }
        count as f32 / (self.num_timepoints * binomial_choose_2(self.num_samples)) as f32
    }
}

impl ModifiedBandDepth {
    pub fn from_samples(rows: &Vec<Vec<f32>>) -> Self {
        let num_samples = rows.len();
        let num_timepoints = rows.first().unwrap().len();
        let mut data_matrix = Vec::with_capacity(num_samples * num_timepoints);
        for j in 0..num_timepoints {
            for i in 0..num_samples {
                data_matrix.push(rows[i][j]);
            }
            data_matrix
                .chunks_mut(num_samples)
                .for_each(|c| c.sort_by(|a, b| a.partial_cmp(b).unwrap()));
        }
        Self {
            num_samples,
            num_timepoints,
            sorted_matrix: data_matrix,
        }
    }
}

/// Computes the number of two-bands
fn binomial_choose_2(from: usize) -> usize {
    match from >= 2 {
        true => ((from * from) - from) / 2,
        false => 0,
    }
}

/// Counts how many elements are `(smaller, equal, larger)` to x
fn partition(sorted_ys: &[f32], x: f32) -> (usize, usize, usize) {
    match sorted_ys.binary_search_by(|e| e.partial_cmp(&x).unwrap()) {
        Result::Err(i) => (i, 0, sorted_ys.len() - i),
        Result::Ok(i) => {
            let (lower, upper) = extents(sorted_ys, x, i);
            (lower, upper - lower, sorted_ys.len() - upper)
        }
    }
}

/// The index found by binary search can be somewhere in the middle,
/// so we need to look left and right to find the beginning and end
/// index of the element that are equal to `x`.
fn extents(sorted: &[f32], x: f32, i: usize) -> (usize, usize) {
    let mut lower = i;
    while lower > 0 && sorted[lower - 1] == x {
        lower -= 1;
    }
    let mut upper = i + 1;
    while upper < sorted.len() && sorted[upper] == x {
        upper += 1;
    }
    (lower, upper)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binomial_choose_2() {
        assert_eq!(binomial_choose_2(5), 10);
        assert_eq!(binomial_choose_2(2), 1);
        assert_eq!(binomial_choose_2(3), 3);
        assert_eq!(binomial_choose_2(10_000), 49_995_000);
        assert_eq!(binomial_choose_2(10_001), 50_005_000);
    }

    #[test]
    fn test_extents() {
        let seq = [1.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 5.0, 8.0, 9.0];
        assert_eq!(extents(&seq, 4.0, 4), (4, 7));
        assert_eq!(extents(&seq, 4.0, 5), (4, 7));
        assert_eq!(extents(&seq, 4.0, 6), (4, 7));

        assert_eq!(extents(&seq, 1.0, 0), (0, 2));
        assert_eq!(extents(&seq, 1.0, 1), (0, 2));

        assert_eq!(extents(&seq, 2.0, 2), (2, 3));

        assert_eq!(extents(&seq, 9.0, 9), (9, 10));
    }

    #[test]
    fn test_partition() {
        let seq = [1.0, 1.0, 2.0, 3.0, 4.0, 4.0, 4.0, 5.0, 8.0, 9.0];
        assert_eq!(partition(&seq, 0.0), (0, 0, 10));
        assert_eq!(partition(&seq, 1.0), (0, 2, 8));
        assert_eq!(partition(&seq, 2.0), (2, 1, 7));
        assert_eq!(partition(&seq, 3.0), (3, 1, 6));
        assert_eq!(partition(&seq, 4.0), (4, 3, 3));
        assert_eq!(partition(&seq, 5.0), (7, 1, 2));
        assert_eq!(partition(&seq, 6.0), (8, 0, 2));
        assert_eq!(partition(&seq, 7.0), (8, 0, 2));
        assert_eq!(partition(&seq, 8.0), (8, 1, 1));
        assert_eq!(partition(&seq, 9.0), (9, 1, 0));
        assert_eq!(partition(&seq, 10.0), (10, 0, 0));
    }

    #[test]
    fn test_from_samples() {
        let data = vec![vec![4.0, 5.0, 6.0], vec![1.0, 2.0, 3.0]];
        let mbd = ModifiedBandDepth::from_samples(&data);
        assert_eq!(
            mbd,
            ModifiedBandDepth {
                num_samples: 2,
                num_timepoints: 3,
                sorted_matrix: vec![1.0, 4.0, 2.0, 5.0, 3.0, 6.0],
            }
        );
    }

    #[test]
    fn test_fully_contained() {
        let data = vec![vec![4.0, 5.0, 6.0], vec![1.0, 2.0, 3.0]];
        let mbd = ModifiedBandDepth::from_samples(&data);
        assert_eq!(mbd.query(&[2.0, 3.0, 4.0]), 1.0);
    }
}
