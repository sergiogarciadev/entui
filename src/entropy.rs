use anyhow::Result;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub fn calculate_shannon_entropy(data: &[u8]) -> f64 {
    if data.is_empty() {
        return 0.0;
    }

    let mut frequencies = [0usize; 256];
    for &byte in data {
        frequencies[byte as usize] += 1;
    }

    let len = data.len() as f64;
    let mut entropy = 0.0;

    for &count in frequencies.iter() {
        if count > 0 {
            let p = count as f64 / len;
            entropy -= p * p.log2();
        }
    }

    entropy
}

pub fn analyze_file(path: &Path, block_size: usize) -> Result<Vec<(u64, f64)>> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = vec![0; block_size];
    let mut results = Vec::new();
    let mut offset = 0u64;

    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }

        let entropy = calculate_shannon_entropy(&buffer[..n]);
        results.push((offset, entropy));
        offset += n as u64;
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_entropy() {
        let data = [0u8; 100];
        assert_eq!(calculate_shannon_entropy(&data), 0.0);
    }

    #[test]
    fn test_max_entropy() {
        let mut data = Vec::new();
        for i in 0..=255 {
            data.push(i as u8);
        }
        let entropy = calculate_shannon_entropy(&data);
        assert!((entropy - 8.0).abs() < 1e-6);
    }
}
