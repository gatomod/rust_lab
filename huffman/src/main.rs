use std::collections::HashMap;
use std::{fs, io};

fn main() -> io::Result<()> {
    // Read directory
    let file_set = fs::read_dir("data/txt")?;

    for file in file_set {
        let path = file?.path();

        if path.is_file() {
            huffman(path)?;
        }
    }

    Ok(())
}

fn huffman(path: std::path::PathBuf) -> Result<(), io::Error> {
    // This code is too similar to Shannon

    // Read file and split by word
    let data = fs::read(&path)?;

    let words: Vec<String> = String::from_utf8_lossy(&data)
        .split_whitespace()
        .map(|x| x.to_string())
        .collect();

    // HashMap with word and counter
    let mut prev_dict: HashMap<String, u64> = HashMap::new();

    for word in &words {
        *prev_dict.entry(word.clone()).or_insert(0) += 1;
    }

    let mut dict: Vec<(String, u64)> = prev_dict.into_iter().map(|x| x).collect();

    dict.sort_by(|(_, a), (_, b)| b.cmp(a));

    Ok(())
}
