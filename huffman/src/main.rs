use std::cell::Cell;
use std::fmt::Debug;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use std::{fs, io, thread};

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
type CounterType = u32;

fn huffman(data: &mut Vec<u8>) -> Result<(), io::Error> {
    println!("BENCH > Start");
    let bench = Instant::now();

    // This is inefficient sorry
    let mut _map = Arc::new(RwLock::new(Vec::<Box<CounterType>>::with_capacity(
        DataType::MAX as usize,
    )));

    println!("BENCH > Initialize map: {:?}", bench.elapsed());

    _map.write()
        .unwrap()
        .par_iter_mut()
        .for_each(|x| *x = Box::new(0));

    println!("BENCH > Fill map with zeroes: {:?}", bench.elapsed());

    println!("{:?}", _map);

    let data: Vec<DataType> = data
        .par_chunks(4)
        .into_par_iter()
        .map(|x| DataType::from_be_bytes([x[0], x[1], x[2], x[3]]))
        .collect();

    println!("BENCH > Data into u16: {:?}", bench.elapsed());

    data.par_iter().for_each(|x| {
        /* *_map
        .get(*x as usize)
        .unwrap_or(&Box::new(RwLock::new(0)))
        .write()
        .unwrap() += 1; */
        if let Some(v) = _map.read().unwrap().get(*x as usize) {
            *v += 1
        }
    });

    // *map.entry(word.clone()).or_insert(0) += 1;
    println!("BENCH > Dictionary created: {:?}", bench.elapsed());

    println!("{:?}", _map);

    println!("BENCH > End with: {:?}", bench.elapsed());

    thread::sleep(Duration::from_secs(100));

    Ok(())
}
