use std::fs;

fn main() {
    // Analyse this solution later: https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2024/day09.rs
    let file_content = fs::read_to_string("./inps/5.txt").unwrap();
    let sample_data = "2333133121414131402".to_string();
    // length of file, length of free space : alternating
    // each also has a file ID based on their initial ordering
    // (ID from initial position, length of file, amt of free space)
}
