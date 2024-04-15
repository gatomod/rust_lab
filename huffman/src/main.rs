use std::borrow::{Borrow, BorrowMut};
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::fmt::{Debug, Display};
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::time::Instant;
use std::{fs, io};

fn main() -> io::Result<()> {
    // Read directory
    let file_set = fs::read_dir("data/img")?;

    for file in file_set {
        let path = file?.path();

        if path.ends_with("wallpaper.raw") {
            let mut data = fs::read(&path)?;
            huffman(&mut data)?;
        }
    }

    Ok(())
}

#[derive(Debug)]
struct Node<T> {
    freq: u64,
    val: Box<Option<T>>,
}

type DataType = u8;

fn huffman(mut data: &mut Vec<u8>) -> Result<(), io::Error> {
    println!("{}", data.len());
    println!("BENCH > Start");
    // Start of simple benchmark
    let bench = Instant::now();

    let mut dict = occurrences(data);

    // Try with HashMap
    /* let mut map: HashMap<DataType, u64> = HashMap::new();

    // Try with
    // let mut map: BTreeMap<DataType, u64> = BTreeMap::new();

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

    for i in &dict {
        println!("{:?}", i);
    }

    println!("BENCH > End with: {:?}", bench.elapsed());

    Ok(())
}

// what tf is this kill me
fn occurrences<T: PartialEq + Copy + Display + Debug>(data: &Vec<T>) -> Vec<Node<T>> {
    let base: Box<Vec<Box<Option<T>>>> =
        Box::new(data.into_iter().map(|x| Box::new(Some(*x))).collect());

    let mut collected: Vec<Node<T>> = Vec::new();

    let mut index = 0usize;
    let base_len = base.len();

    for query in &*base {
        for element_index in index..base_len {
            if let Some(mut boxed_val) = base.clone().get_mut(element_index) {
                collected.push(Node {
                    freq: 1,
                    val: boxed_val.clone(),
                });

                boxed_val = &mut Box::new(None);
            }
        }

        index += 1;
    }

    collected
}
