# 🧪 Rust lab

Testing with Rust

This is a monorepo with projects that I make to test algorithms and other things in Rust.

## Contributors
Thanks to my cat for helping me reviewing the code and stepping on the keyboard while I wasn't at my desk.

## Crates

-   **Shannon:** A simple Shannon entropy implementation
-   **Huffman:** A Huffman tree generator

## Dataset

All test data is available under the /data folder. There's a lot of garbage, like weird texts and images (with raw and encoded data). Don't take it personally.

## Crates information

### Shannon

This is a simple implementation of the Shannon entropy algorithm. It takes some files from the dataset, reads each byte and calculates its probability and data entropy. You can also calculate it by words if you're working with words.

### Huffman

An implementation of the Huffman coding and tree generator with efficient dataset management. As the previous project, it takes data from the dataset.

Collecting data and probability of appearance is too inefficient with large data. Currently I'm using a hashmap because is easy to check if the entry is present in the list. Hashmap is too slow, but still faster than using a vector with an iterator that loops the dataset for each entry. I should create a custom binary tree that can find and store data efficiently.

_Gátomo - The 3-Clause BSD License_
