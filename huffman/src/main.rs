use std::borrow::{Borrow, BorrowMut};
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Display};
use std::io::Read;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;
use std::time::Instant;
use std::{fs, io};

use rayon::prelude::*;

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

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Node<T: Ord> {
    freq: u64,
    val: Option<T>,
}

type DataType = u32;

fn huffman(mut data: &mut Vec<u8>) -> Result<(), io::Error> {
    println!("BENCH > Start");
    let bench = Instant::now();

    let mut map = Arc::new(HashMap::<DataType, u64>::new());

    let data: Vec<DataType> = data
        .par_chunks(4)
        .into_par_iter()
        .map(|x| DataType::from_be_bytes([x[0], x[1], x[2], x[3]]))
        .collect();

    println!("BENCH > Data into u16: {:?}", bench.elapsed());

    data.par_iter().for_each(|x| {
        map.entry(*x).or_insert(0) += 1;
    });

    // *map.entry(word.clone()).or_insert(0) += 1;
    println!("BENCH > Dictionary created: {:?}", bench.elapsed());

    println!("BENCH > End with: {:?}", bench.elapsed());

    Ok(())
}
