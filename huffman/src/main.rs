use std::collections::HashMap;
use std::{fs, io};

fn main() -> io::Result<()> {
    // Read directory
    let file_set = fs::read_dir("data/txt")?;

    for file in file_set {
        let path = file?.path();

        if path.ends_with("song.txt") {
            let data = fs::read(&path)?;
            huffman(data)?;
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Leaf {
    freq: u64,
    val: u8,
}

#[derive(Debug)]
struct Branch<'a> {
    freq: u64,
    a: Box<&'a Leaf>,
    b: Box<&'a Leaf>,
}

#[derive(Debug)]
enum Tree<'a> {
    Leaf(Leaf),
    Branch(Branch<'a>),
}

trait TreeFrequency {
    fn get_freq(&self) -> u64;
}

impl<'a> TreeFrequency for Tree<'a> {
    fn get_freq(&self) -> u64 {
        match self {
            Tree::Leaf(x) => x.freq,
            Tree::Branch(x) => x.freq,
        }
    }
}

fn huffman(data: Vec<u8>) -> Result<(), io::Error> {
    // HashMap with word and counter
    let mut prev_dict: HashMap<u8, u64> = HashMap::new();

    for word in &data {
        *prev_dict.entry(word.clone()).or_insert(0) += 1;
    }

    // Create dictionary with all leaves and occurrences
    let mut dict: Vec<Leaf> = prev_dict
        .into_iter()
        .map(|(val, freq)| Leaf { freq, val })
        .collect();

    let mut 

    dict.sort_by(|a, b| b.freq.cmp(&a.freq));

    for i in dict {
        println!("{:?}", i);
    }

    Ok(())
}
