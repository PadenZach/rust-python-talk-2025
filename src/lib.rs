use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

use rayon::prelude::*;
use rayon::ThreadPoolBuilder;

/// Build a 256-bin histogram after linear normalization to [0, 255].
/// - nums: slice of nonnegative integers (u32).
/// - returns: [u32; 256] counts.
pub fn histogram_256(nums: &[u32]) -> [u32; 256] {
    if nums.is_empty() {
        return [0u32; 256];
    }

    // Pass 1: compute min and max (sequential is fine; could also parallel-reduce).
    let (&mn, &mx) = (
        nums.iter().min().expect("non-empty"),
        nums.iter().max().expect("non-empty"),
    );

    // Edge case: all equal → everything maps to bin 0.
    if mx == mn {
        let mut hist = [0u32; 256];
        hist[0] = nums.len() as u32;
        return hist;
    }

    let range = (mx - mn) as u64;

    // Pass 2: parallel chunking → local histograms → reduce without data races.
    nums.par_chunks(1 << 16) // tune chunk size to taste
        .map(|chunk| {
            let mut local = [0u32; 256];
            for &x in chunk {
                // Promote to u64 to avoid overflow in (x - mn) * 255
                let idx = (((x - mn) as u64) * 255) / range;
                // idx ∈ [0,255], safe to cast to usize
                local[idx as usize] += 1;
            }
            local
        })
        .reduce(|| [0u32; 256], |mut a, b| {
            // Elementwise add – no races, all local
            for i in 0..256 {
                a[i] = a[i].saturating_add(b[i]); // saturating for belt-and-suspenders safety
            }
            a
        })
}

/// Python wrapper for histogram_256.
///
/// Accepts a Python iterable/list of integers (i64). Negative values will raise ValueError.
/// Returns a Python list of 256 unsigned counts (u32).
#[pyfunction]
fn histogram_256_py(py_nums: Vec<i64>) -> PyResult<Vec<u32>> {
    // Convert and validate input
    let mut nums: Vec<u32> = Vec::with_capacity(py_nums.len());
    for n in py_nums {
        if n < 0 {
            return Err(PyValueError::new_err("negative numbers are not allowed"));
        }
        nums.push(n as u32);
    }

    let hist = histogram_256(&nums);
    Ok(hist.to_vec())
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn histogram_256_from_file(path: &str) -> PyResult<Vec<u32>> {
    use std::fs::File;
    use std::io::{BufRead, BufReader};

    let file = File::open(path)
        .map_err(|e| PyValueError::new_err(format!("failed to open file '{}': {}", path, e)))?;
    let reader = BufReader::new(file);
    let mut nums: Vec<u32> = Vec::new();

    for line_res in reader.lines() {
        let line = line_res
            .map_err(|e| PyValueError::new_err(format!("failed to read line: {}", e)))?;
        let s = line.trim();
        if s.is_empty() {
            continue;
        }
        let val: i64 = s.parse().map_err(|e| {
            PyValueError::new_err(format!("failed to parse integer '{}': {}", s, e))
        })?;
        if val < 0 {
            return Err(PyValueError::new_err("negative numbers are not allowed"));
        }
        nums.push(val as u32);
    }

    let hist = histogram_256(&nums);
    Ok(hist.to_vec())
}


#[pymodule]
fn rust_python_talk_2025(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Try to initialize a global Rayon thread pool. If another part of the program
    // (or Python embedding) already initialized it, ignore the error.
    let _ = ThreadPoolBuilder::new().build_global();

    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(histogram_256_py, m)?)?;
    m.add_function(wrap_pyfunction!(histogram_256_from_file, m)?)?;
    Ok(())
}
