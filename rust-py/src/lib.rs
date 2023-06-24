use pyo3::prelude::*;

fn zero_matrix(n: usize) -> Vec<Vec<usize>> {
    let mut m = Vec::with_capacity(n);
    for _ in 0..n {
        m.push(vec![0; n]);
    }
    m
}

#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// Multiplies two n by n matrices.
/// Naively assumes the matrices are both n by n.
#[pyfunction]
fn matmul(a: Vec<Vec<usize>>, b: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut c = zero_matrix(n);
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

/// A Python module implemented in Rust.
#[pymodule]
fn rust_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(matmul, m)?)?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
