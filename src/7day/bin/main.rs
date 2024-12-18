use itertools::Itertools;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn valid_sum(t: i32, nums: Vec<i32>) -> (bool, i32) {
    let temp = nums.iter().sum::<i32>();
    match temp == t {
        true => (true, temp),
        _ => (false, 0),
    }
}

/*fn work() {

}
*/

fn main() {
    let mut ans = 0;
    let operators = ("*", "+");
    if let Ok(lines) = read_lines("./inps/7.txt") {
        for line in &mut lines.map_while(Result::ok) {
            //flatten() {
            //println!("{}", line);
            // Parse each line here
            //let args_vec: Vec<i32> = Vec::new();
            let mut line_iter = line.split(":");
            if let (Some(y), Some(args)) = (line_iter.next(), line_iter.next()) {
                let target = y.parse::<i32>().ok();
                let args_vec: Vec<i32> = args
                    .split_whitespace()
                    .filter_map(|x| x.parse::<i32>().ok())
                    .collect();

                // lines prepared now see if the condition can be satisfied:
                // Helper function here?
                // brute force generate all combinations?
                let length = args_vec.clone().len();
                let all_combos = args_vec.into_iter().combinations(length);
                //.collect();
                //print_type_of(&all_combos);

                for each in all_combos {
                    //dbg!("{:?}", &each);
                    if let Some(valid_t) = target {
                        let (temp, val) = valid_sum(valid_t, each);

                        if temp {
                            ans += val;
                        }
                    }
                }
            }
            println!("{ans}");
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_result() {
        todo!() //assert_eq!(
    }
}
