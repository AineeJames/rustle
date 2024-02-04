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

/// takes an encoded str like a4b3c2 and returns the decoded string like aaaabbbcc
#[pyfunction]
fn decode_str(input: &str) -> String {
    let mut result = String::new();
    for (i, c) in input.chars().enumerate() {
        if c.is_ascii_digit() {
            continue;
        }
        let mut count = String::from("0");
        for d in input.chars().skip(i + 1) {
            if d.is_ascii_digit() {
                count.push(d);
            } else {
                break;
            }
        }
        let count = count.parse::<usize>().unwrap();
        for _ in 0..count {
            result.push(c);
        }
    }
    result
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustle(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_str, m)?)?;
    m.add_function(wrap_pyfunction!(decode_str, m)?)?;
    Ok(())
}
