use pyo3::buffer::PyBuffer;
use pyo3::prelude::*;

#[pyfunction]
fn buffer_len(obj: &PyAny) -> PyResult<usize> {
    let buffer = PyBuffer::<u8>::get(obj)?;
    Ok(buffer.item_size() * buffer.item_count())
}

#[pymodule]
fn repro(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(buffer_len, m)?)?;
    Ok(())
}
