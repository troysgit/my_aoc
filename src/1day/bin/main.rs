use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// `read_lines` taken from Rust By Example at,
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    // read input
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("inps/1.txt") {
        for line in lines.map_while(Result::ok) {
            //lines.flatten() replace with map_while(Result::ok);
            let mut split = line.split_whitespace();
            if let (Some(w1), Some(w2)) = (split.next(), split.next()) {
                if let (Ok(n1), Ok(n2)) = (w1.parse::<i32>(), w2.parse::<i32>()) {
                    col1.push(n1);
                    col2.push(n2);
                }
            }
        }
    }
    dbg!(&col1);
    dbg!(&col2);
    let mut col1_part_one_copy = col1.clone();
    let mut col2_part_one_copy = col2.clone();
    let col1_part_two_copy = col1.clone();
    let col2_part_two_copy = col2.clone();
    part_one(&mut col1_part_one_copy, &mut col2_part_one_copy);
    part_two(col1_part_two_copy, col2_part_two_copy);
}

fn part_one(first_col: &mut [i32], sec_col: &mut [i32]) {
    first_col.sort();
    sec_col.sort();
    let abs_diff: i32 = sec_col
        .iter()
        .zip(first_col.iter())
        .map(|(a, b)| (a - b).abs())
        //.fold(0, |acc, x| acc + x);
        .sum();
    // Correct answer!
    println!("{abs_diff}");
}

fn part_two(one: Vec<i32>, two: Vec<i32>) {
    let hash_set_l: HashSet<i32> = one.iter().cloned().collect();
    let mut coll_ans: i32 = 0;
    for each_val in hash_set_l {
        let temp = two.iter().filter(|&n| *n == each_val).count();
        let prod = temp as i32 * each_val;
        coll_ans += prod;
    }
    // Correct answer!
    println!("{coll_ans}");
}
