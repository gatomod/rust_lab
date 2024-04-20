use std::{fs, io, time::Instant};

#[derive(Debug, Clone)]
struct Node<T: Ord + Eq> {
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
    value: Option<T>,
}

fn main() -> io::Result<()> {
    // Read directory
    let file_set = fs::read_dir("data/img")?;
    let larbv = Box::new("mosca");

    let xd = larbv.as_ptr();

    println!("{:?}", xd);

    for file in file_set {
        let path = file?.path();

        if path.ends_with("_lake.raw") {
            let mut data = fs::read(&path)?;

            arr(&mut data);
        }
    }

    Ok(())
}

fn arr(buf: &mut Vec<u8>) {
    println!("BENCH > Start");
    // Start of simple benchmark
    let bench = Instant::now();

    let mut tree: Vec<Node<u8>> = Vec::new();

    for i in buf {}

    println!("BENCH > End with: {:?}", bench.elapsed());
}

/* fn main() {
    let a = Node {
        left: None,
        right: None,
        value: Some("A"),
    };

    let b = Node {
        left: None,
        right: None,
        value: Some("B"),
    };

    let c = Node {
        left: None,
        right: None,
        value: Some("C"),
    };

    let branch = Node {
        left: Some(Box::new(b)),
        right: Some(Box::new(c)),
        value: None,
    };

    let root = Node {
        left: Some(Box::new(a)),
        right: Some(Box::new(branch)),
        value: None,
    };

    println!("{:#?}", root);
} */
