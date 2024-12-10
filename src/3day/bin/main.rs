use std::io::Read;
use std::fs::File;

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

fn main() { 
    let mut contents = String::new();
    let mut f = File::open("inps/3.txt").expect("open");
    f.read_to_string(&mut contents).expect("open again");
    part_one(contents);
}

#[cfg(test)]
mod test {
    use super::*;
    todo!();
}



