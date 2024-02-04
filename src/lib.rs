use pyo3::prelude::*;

#[pyfunction]
fn encode_str(input: &str) -> String {
    let mut result = String::new();
    let mut prev_char = input.chars().next().unwrap();
    let mut count = 1;
    for c in input.chars().skip(1) {
        if c == prev_char {
            count += 1;
        } else {
            result.push(prev_char);
            result.push_str(&count.to_string());
            count = 1;
            prev_char = c;
        }
    }
    result.push(prev_char);
    result.push_str(&count.to_string());
    result
}

#[pyfunction]
fn decode_str(input: &str) -> String {
    unimplemented!()
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustle(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_str, m)?)?;
    m.add_function(wrap_pyfunction!(decode_str, m)?)?;
    Ok(())
}
