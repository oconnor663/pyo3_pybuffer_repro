use pyo3::buffer::PyBuffer;
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn buffer_len(obj: &PyAny) -> PyResult<usize> {
    let buffer = PyBuffer::<u8>::get(obj)?;
    Ok(buffer.item_size() * buffer.item_count())
}

/// A Python module implemented in Rust.
#[pymodule]
fn repro(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(buffer_len, m)?)?;

    Ok(())
}
