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

#[pyfunction]
fn encode_bw_image(image: Vec<Vec<u8>>) -> Vec<Vec<(u8, u8)>> {
    let mut encoded_image = Vec::new();

    for row in image {
        let mut encoded_row = Vec::new();
        let mut current_pixel = row[0];
        let mut count = 0u8;

        for &pixel in row.iter() {
            if pixel == current_pixel && count < u8::MAX {
                // Increment count if the current pixel matches the previous, and we haven't hit the count limit
                count += 1;
            } else {
                // Otherwise, push the pixel and its count to the encoded row, and reset the count
                encoded_row.push((current_pixel, count));
                current_pixel = pixel;
                count = 1;
            }
        }
        // Don't forget to add the last sequence
        encoded_row.push((current_pixel, count));

        encoded_image.push(encoded_row);
    }

    encoded_image
}

#[derive(FromPyObject)]
struct RustyTuple(u8, u8);

#[pyfunction]
fn decode_bw_image(encoded_image: Vec<Vec<RustyTuple>>) -> Vec<Vec<u8>> {
    let mut decoded_image = Vec::new();
    for row in encoded_image {
        let mut encoded_row: Vec<u8> = Vec::new();
        for rle_item in row {
            for _ in 0..rle_item.1 {
                encoded_row.push(rle_item.0);
            }
        }
        decoded_image.push(encoded_row);
    }
    decoded_image
}

/// A Python module implemented in Rust.
#[pymodule]
fn rustle(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(encode_str, m)?)?;
    m.add_function(wrap_pyfunction!(decode_str, m)?)?;
    m.add_function(wrap_pyfunction!(encode_bw_image, m)?)?;
    m.add_function(wrap_pyfunction!(decode_bw_image, m)?)?;
    Ok(())
}
