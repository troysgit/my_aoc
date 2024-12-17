use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() { 
    let ans = 0;
    if let Ok(lines) = read_lines("./inps/7.txt") {
        for line in &mut lines.map_while(Result::ok) {//flatten() {
            //println!("{}", line);
            // Parse each line here
            //let args_vec: Vec<i32> = Vec::new();
            let mut line_iter = line.split(":"); 
            if let (Some(y), Some(args)) = (line_iter.next(), line_iter.next()) {
                let args_vec: Vec<i32> = args
                    .split_whitespace()
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect();
                

                //for each in &args_vec {
                //    println!("{each}");
                //}
            
            // visual debug    
            //println!("{}", required_sum.unwrap());
            //for val in line_iter { 
            //    println!("{val}");
            //}

            }
        }

    }

}

