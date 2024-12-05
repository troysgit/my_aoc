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
    
    let mut expression_list = Vec::new();
    //dbg!("{contents}");
    let mut iter_chars = sample_data.chars().peekable();
    while let Some(c) = iter_chars.next() {
        match c {
            'm' => todo!(),//check if next() is u
            'u' => todo!(), // check if next() is l
            'l' => todo!(), // check if next() is (
            '(' => todo!(), //check if next is_digit()
            ',' => todo!(), 
            ')' => todo!(),

        }
    }

    Ok(())
}

