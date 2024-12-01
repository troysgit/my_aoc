use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    // read input
    let mut col1: Vec<i32> = Vec::new();
    let mut col2: Vec<i32> = Vec::new();
    if let Ok(lines) = read_lines("inps/1.txt") {
        for line in lines.flatten() {
            let mut split = line.split_whitespace();
            if let (Some(w1), Some(w2)) = (split.next(), split.next()) {
                if let (Ok(n1), Ok(n2)) = (w1.parse::<i32>(), w2.parse::<i32>()) {
                    col1.push(n1);
                    col2.push(n2);
                }

            }

        }

    }
    
    col1.sort();
    col2.sort();
    let abs_diff: i32 = col2.iter()
        .zip(col1.iter())
        .map(|(a,b)| (a-b).abs())
        //.fold(0, |acc, x| acc + x);
        .sum();
    // Correct answer!
    println!("{abs_diff}");
    //let ans1 = part_one();
    //let ans2 = part_two();
    

    // part two
    let hash_set_l: HashSet<i32> = col1.iter().cloned().collect();
    let mut coll_ans: i32 = 0; 
    for each_val in hash_set_l {
        let temp = col2.iter().filter(|&n| *n == each_val).count();
        let prod = temp as i32 * each_val;
        coll_ans += prod;
    }
    println!("{coll_ans}");


        

}

fn part_one() -> Option<i32> {
    None
}


fn part_two() -> Option<i32> {
    None
}

