use std::{collections::HashMap, fs, io};

fn main() -> io::Result<()> {
    // Read directory
    let file_set = fs::read_dir("data/img")?;

    for file in file_set {
        let path = file?.path();

        if path.ends_with("_lake.raw") {
            shannon(path)?;
        }
    }

    Ok(())
}

fn shannon(path: std::path::PathBuf) -> Result<(), io::Error> {
    // Read file and split by word
    let data = fs::read(&path)?;

    /* let words: Vec<String> = String::from_utf8_lossy(&data)
    .split_whitespace()
    .map(|x| x.to_string())
    .collect(); */

    /*
    HashMap with word, counter and probability.

    If word doesn't exists, creates a new one and then increment the counter
    */
    let mut dict: HashMap<u8, (u64, f64)> = HashMap::new();

    for word in &data {
        (*dict.entry(word.clone()).or_insert((0u64, 0f64))).0 += 1;
    }

    // Calculate the word frequency and works out with the Shannon entropy
    let mut entropy = 0f64;

    for (_, (count, prob)) in &mut dict {
        // Get the probability of appearance in the dataset
        *prob = *count as f64 / data.len() as f64;

        // Multiply the probability with the base 2 logarithm of itself
        // and calculate the summation of each probability
        entropy += -(*prob * prob.log2());
    }

    // Print results
    for (word, (count, prob)) in &dict {
        println!("{:<20}\t{}\t{:<22}%", word, count, prob);
    }

    println!(
        "\n{}\n- Entropy: {}\n- Data length: {}\n- Map length: {}",
        path.to_string_lossy(),
        entropy,
        data.len(),
        dict.len()
    );

    Ok(())
}
