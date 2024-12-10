use std::io::Read;
use std::fs::File;
use std::slice::Windows;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

fn part_one(input_data: String) { 
    let final_step: i32 = 
        input_data
        .split("mul(")
        .map(|option| {
            option
                .chars()
                .take_while(|&c| c != ')')
                .collect::<String>()
                .split_once(',')
                .and_then(|(a,b)| {
                    a.trim().parse::<i32>().ok().zip(b.trim().parse::<i32>().ok())
                })
                .map(|(a,b)| a*b)
                .unwrap_or(0)
        }).sum();
        //.filter(|&x| x != 0);
        //.sum()
        print_type_of(&final_step);
        println!("{final_step}");
}


fn part_two(input_data: String) -> i32 { 
    let mut cnt = 0;
    let mut do_mode: bool = true;
    let tokens: i32 = 
        input_data
        .split(|c| c == 'd' || c == 'o')
        .filter(|&token| !token.is_empty())
        .map(|token| {
            if token.contains("do()") {
                do_mode = true;
                return 0;
            } else if token.contains("don't()") {
                do_mode = false;
                return 0;
            } 
            if do_mode {
                parse_mul(token)
            } else {
                return 0;
            }
        }).sum();
    //println!("{tokens}");
    tokens
}

fn part_one_elegant(mut input_data: String) {
    //let mut slice = &input_data[..];
    let cnt: i32 = input_data
        //.std::slice::windows(4)
        .char_indices()
        //.enumerate()
        .filter_map(|(i, _)| {
            if input_data[i..].starts_with("mul(") {
               input_data[i+4..]
                   .split(')')
                   .next()
                   .and_then(|sub| {
                       let mut parts = sub.split(',');
                       let a = parts.next()?.parse::<i32>().ok()?;
                       let b = parts.next()?.parse::<i32>().ok()?;
                       Some(a*b)
                   })
            } else {
                None
            }
        })
    .sum();
    println!("{cnt}");
}


fn parse_mul(section: &str) -> i32 {
    let ans = section
        .split("mul(")
        .filter_map(|part| {
            let cleaned = part 
                .chars()
                .take_while(|&c| c != ')')
                .collect::<String>();
            cleaned
                .split_once(',')
                .and_then(|(a, b)| {
                    a.trim()
                        .parse::<i32>()
                        .ok()
                        .zip(b.trim().parse::<i32>().ok())
                })
                .map(|(a, b)| a * b) 
            })
        .sum();
    ans
}  

fn main() { 
    let mut contents = String::new();
    let mut f = File::open("inps/3.txt").expect("open");
    f.read_to_string(&mut contents).expect("open again");
    let part_two_inp = contents.clone();
    let part_one_ele_inp = contents.clone();
    //part_one(contents);
    //part_two(part_two_inp);
    part_one_elegant(part_one_ele_inp);
}

#[cfg(test)]
mod test {
    use super::*;
    todo!();
}



