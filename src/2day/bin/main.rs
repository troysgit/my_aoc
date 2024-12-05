use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// `read_lines` taken from Rust By Example at,
// https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() { 
    let mut _ans = 0;
    if let Ok(lines) = read_lines("inps/2.txt") {
        let mut ans = 0;
        for line in lines.map_while(Result::ok) {
            //let mut line = String::new();
            //std::io::stdin().read_line(&mut lines).expect("integer input");
            let nums = line.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();
            let init_diff = nums[1]-nums[0];
            let mut valid_incr = true;
            let mut valid_decr = true;
            
            if init_diff > 0 && init_diff < 4{
                // increasing 
                for idx in 1..nums.len()-1 {
                    if nums[idx+1] - nums[idx] > 3 || nums[idx+1] - nums[idx] <= 0 {
                        valid_incr= false;
                    }
                }
                if valid_incr {
                    ans += 1;
            
                }
            
            } else if init_diff < 0 && init_diff > -4{
                // decreasing
                for idx in 1..nums.len()-1 {
                    if nums[idx] - nums[idx+1] > 3 || nums[idx] - nums[idx+1] <= 0 {
                        valid_decr = false;
                    }
                }
                if valid_decr {
                    ans += 1;
                }
            } else {
                continue;
            }
        }
    // Wrong answer at this stage
    println!("{ans}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;


}
