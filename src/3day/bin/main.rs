use std::fs::File;
use std::io::Read;
use std::error::Error;

fn main()-> Result<(), Box<dyn Error>> { 
    let sample_data: String = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string();
    let mut contents = String::new();
    let mut f = File::open("inps/3.txt").expect("open");
    f.read_to_string(&mut contents).expect("open again");
    //contents.chars()
    //    .for_each(|c| println!("{}", c));
    
    let mut expression_list: Vec<i32> = Vec::new();
    //dbg!("{contents}");
    let mut iter_chars = sample_data.chars().peekable();
    while let Some(c) = iter_chars.next() {
        match c {
            'm' => if iter_chars.clone().take(2).collect::<String>() == "ul" {
                if let Some('(') = iter_chars.next() { 
                    let mut first_val = String::new();

                    while let Some(c) = iter_chars.next() {
                        if c.is_digit(10) { // radix of 10
                            first_val.push(c);
                    } else if c == ',' {
                        break;
                    } else {
                        break;
                    }
                }
                let mut second_val = String::new();
                while let Some(c) = iter_chars.next() { 
                    if c.is_digit(10) {
                        second_val.push(c);
                    } else if c == ')' {
                        break;
                    } else {
                        break;
                    }
                }
                if !first_val.is_empty() && !second_val.is_empty() {
                    expression_list.push(first_val.parse::<i32>().unwrap() * second_val.parse::<i32>().unwrap());
                }
                }

            }
            _ => println!("oh oh"),

        }
    }
    let temp_ans: i32  = expression_list.iter().fold(0, |acc, val| acc + val);
    println!("{temp_ans}");
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    todo!();
}



