use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::fmt::{Debug, Display};
use std::time::Instant;
use std::{fs, io};

fn main() -> io::Result<()> {
    // Read directory
    let file_set = fs::read_dir("data/img")?;

    for file in file_set {
        let path = file?.path();

        if path.ends_with("_lake.raw") {
            let mut data = fs::read(&path)?;
            huffman(&mut data)?;
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Node<T> {
    freq: u64,
    val: Option<T>,
}

type DataType = u8;

fn huffman(mut data: &mut Vec<u8>) -> Result<(), io::Error> {
    println!("{}", data.len());
    println!("BENCH > Start");
    // Start of simple benchmark
    let bench = Instant::now();

    let mut dict = occurrences(data);

    // Try with HashMap
    // let mut map: HashMap<DataType, u64> = HashMap::new();

    // Try with
    /* let mut map: BTreeMap<DataType, u64> = BTreeMap::new();

    for word in data {
        *map.entry(word.clone()).or_insert(0) += 1;
    }

    println!("BENCH > Add entries with hashmap: {:?}", bench.elapsed());

    let mut dict: Vec<Node<DataType>> = map
        .into_iter()
        .map(|(val, freq)| Node {
            freq,
            val: Some(val),
        })
        .collect(); */

    println!("BENCH > Dictionary created: {:?}", bench.elapsed());

    dict.sort_by(|a, b| b.freq.cmp(&a.freq));

    println!("BENCH > Sorted: {:?}", bench.elapsed());
    // let mut dict = dict.into();

    /* let first = dict.first().unwrap();

    let branches: Vec<Node<DataType>> = Vec::with_capacity(dict.len() - 1); */

    /* for i in &dict {
        println!("{:?}", i);
    } */

    println!("BENCH > End with: {:?}", bench.elapsed());

    Ok(())
}

// This SUCKS
fn occurrences<T: PartialEq + Copy + Display + Debug>(data: &mut Vec<T>) -> Vec<Node<T>> {
    let mut dict: Vec<Node<T>> = Vec::new();

    let mut i = 0;
    while !data.is_empty() {
        let bench = Instant::now();
        let el = data.pop().unwrap();

        let mut exists = false;

        if let Some(dict_val) = dict.iter_mut().find(|x| x.val == Some(el)) {
            dict_val.freq += 1;
            exists = true;
        } else {
            let node = Node {
                freq: 1,
                val: Some(el.clone()),
            };

            dict.push(node);
        }

        i += 1;
        if i % 1_000_000 == 0 {
            println!("Element: {} - Exists: {}", el, exists);

            println!("BENCH > Loop {} with: {:?}", i, bench.elapsed());
        }
    }

    dict
}
