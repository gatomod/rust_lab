use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::time::Instant;
use std::{fs, io};

fn main() -> io::Result<()> {
    // Read directory
    let file_set = fs::read_dir("data/img")?;

    for file in file_set {
        let path = file?.path();

        if path.ends_with("wallpaper.raw") {
            let data = fs::read(&path)?;
            huffman(data)?;
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Leaf {
    freq: u64,
    val: Option<u8>,
}

fn huffman(data: Vec<u8>) -> Result<(), io::Error> {
    println!("BENCH > Start");
    // Start of simple benchmark
    let bench = Instant::now();

    // TODO find an efficient way to collect data, this takes 3 seconds with large datasets

    // HashMap with word and counter
    /* let mut occurrences: Vec<(u8, u64)> = Vec::new();

    'word: for word in &data {
        for (val, mut oc) in &mut occurrences {
            if word == val {
                oc += 1;
                continue 'word;
            }
        }

        occurrences.push((*word, 1))
    } */

    /* let mut prev_dict: HashMap<u8, u64> = HashMap::new();

    for word in &data {
        *prev_dict.entry(word.clone()).or_insert(0) += 1;
    } */

    println!("BENCH > Add entries: {:?}", bench.elapsed());

    // Create dictionary with all leaves and occurrences
    let mut dict: Vec<Leaf> = occurrences
        .into_iter()
        .map(|(val, freq)| Leaf {
            freq,
            val: Some(val),
        })
        .collect();

    println!("BENCH > Dictionary created: {:?}", bench.elapsed());

    dict.sort_by(|a, b| b.freq.cmp(&a.freq));

    println!("BENCH > Sorted: {:?}", bench.elapsed());
    // let mut dict = dict.into();

    let first = dict.first().unwrap();

    let branches: Vec<Leaf> = Vec::with_capacity(dict.len() - 1);

    /* for i in &dict {
        println!("{:?}", i);
    } */

    println!("BENCH > End with: {:?}", bench.elapsed());

    Ok(())
}
