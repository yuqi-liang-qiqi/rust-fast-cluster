//! This library provides a hierarchical clustering implementation using Ward's method.
//! - Use `linkage(...)` in native Rust.
//! - Use `linkage_py(...)` via Python with PyO3 + numpy.

use ndarray::Array2;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;
use ordered_float::NotNan;

// --- Rust-native function ---
pub fn linkage(d: &[f64], method: &str) -> Array2<f64> {
    assert_eq!(method, "ward", "Only 'ward' method is supported");

    let m = d.len();
    let n = ((1.0 + (1.0 + 8.0 * m as f64).sqrt()) / 2.0).round() as usize;
    assert_eq!(m, n * (n - 1) / 2, "Invalid condensed matrix length");

    let mut sizes = vec![1; 2 * n - 1];
    let mut active = vec![true; 2 * n - 1];
    let mut heap = BinaryHeap::<Reverse<(NotNan<f64>, usize, usize)>>::new();
    let mut new_dists = HashMap::new();

    for i in 0..n {
        for j in (i + 1)..n {
            let dist = d[condensed_index(i, j, n)];
            assert!(dist.is_finite(), "Non-finite distance found in input.");
            heap.push(Reverse((NotNan::new(dist).unwrap(), i, j)));
        }
    }

    let mut z = Array2::<f64>::zeros((n - 1, 4));

    for step in 0..(n - 1) {
        let (a, b, dist) = find_closest_pair(&mut heap, &active);
        let new_idx = n + step;

        z[[step, 0]] = a as f64;
        z[[step, 1]] = b as f64;
        z[[step, 2]] = dist;
        z[[step, 3]] = (sizes[a] + sizes[b]) as f64;

        sizes[new_idx] = sizes[a] + sizes[b];
        active[a] = false;
        active[b] = false;
        active[new_idx] = true;

        update_distances(d, &sizes, &mut heap, &active, new_idx, a, b, n, &mut new_dists);
    }

    z
}

// Python wrapper
use numpy::{IntoPyArray, PyArray2, PyReadonlyArray1};
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(name = "linkage")]
fn linkage_py<'py>(
    py: Python<'py>,
    d_py: PyReadonlyArray1<'py, f64>,
    method: &str,
) -> &'py PyArray2<f64> {
    let d = d_py.as_slice().unwrap();
    linkage(d, method).into_pyarray(py)
}

#[pymodule]
fn _rust_fast_cluster(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(linkage_py, m)?)?;
    Ok(())
}

// Helper Functions
/// Efficiently find the index into the 1D condensed form of the upper triangle distance matrix
fn condensed_index(i: usize, j: usize, n: usize) -> usize {
    assert!(i < j);
    i * (2 * n - i - 3) / 2 + j - 1
}

/// Safely retrieve the distance between two clusters (from original or new dists)
fn get_distance(
    i: usize,
    j: usize,
    n: usize,
    d: &[f64],
    new_dists: &HashMap<(usize, usize), f64>,
) -> f64 {
    let key = (i.min(j), i.max(j));
    if key.0 < n && key.1 < n {
        d[condensed_index(key.0, key.1, n)]
    } else {
        *new_dists.get(&key).unwrap_or(&0.0)
    }
}

/// Find the closest pair of clusters using a min-heap
fn find_closest_pair(
    heap: &mut BinaryHeap<Reverse<(NotNan<f64>, usize, usize)>>,
    active: &[bool],
) -> (usize, usize, f64) {
    while let Some(Reverse((dist, i, j))) = heap.pop() {
        if active[i] && active[j] {
            return (i, j, dist.into_inner());
        }
    }
    panic!("No valid cluster pair found.");
}

/// Update distances for new cluster using Ward linkage
fn update_distances(
    d: &[f64],
    sizes: &[usize],
    heap: &mut BinaryHeap<Reverse<(NotNan<f64>, usize, usize)>>,
    active: &[bool],
    new_idx: usize,
    a: usize,
    b: usize,
    n: usize,
    new_dists: &mut HashMap<(usize, usize), f64>,
) {
    let size_a = sizes[a] as f64;
    let size_b = sizes[b] as f64;
    let size_ab = size_a + size_b;

    for i in 0..new_idx {
        if !active[i] || i == a || i == b {
            continue;
        }

        let d_ai = get_distance(i, a, n, d, new_dists);
        let d_bi = get_distance(i, b, n, d, new_dists);
        let d_ab = get_distance(a, b, n, d, new_dists);
        let size_i = sizes[i] as f64;

        let numerator = (size_a + size_i) * d_ai.powi(2)
            + (size_b + size_i) * d_bi.powi(2)
            - size_i * d_ab.powi(2);
        let denominator = size_ab + size_i;
        let value = numerator / denominator;

        if !value.is_finite() || value < 0.0 {
            continue;
        }

        let new_dist = value.sqrt();
        new_dists.insert((i.min(new_idx), i.max(new_idx)), new_dist);
        heap.push(Reverse((NotNan::new(new_dist).unwrap(), i.min(new_idx), i.max(new_idx))));
    }
}

