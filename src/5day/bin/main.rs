use std::fs;
fn is_ready(ordering_rules: &[(i32, i32)], page_nums: &[i32]) -> bool {
    for (a, b) in ordering_rules.iter() {
        let mut first_ind = None;
        let mut second_ind = None;
        for (idx, val) in page_nums.iter().enumerate() {
            if *a == *val {
                first_ind = Some(idx);
            } else if *b == *val {
                second_ind = Some(idx);
            }
        }
        match (first_ind, second_ind) {
            (Some(first), Some(second)) => {
                if first > second {
                    return false;
                }
            }
            (None, None) => continue,          //return true,
            (None, Some(_second)) => continue, //return true,
            (Some(_first), None) => continue,  //return true,
            _ => return false,
        }
    }
    true
}

fn process_orders(file: String) -> Vec<(i32, i32)> {
    let mut orders: Vec<(i32, i32)> = Vec::new();
    for line in file.lines() {
        if let Some(pos) = line.find('|') {
            let (left, right) = line.split_at(pos);

            if let (Ok(a), Ok(b)) = (left.parse::<i32>(), right[1..].parse::<i32>()) {
                orders.push((a, b));
            }
        }
    }
    orders
}

fn process_pages(file: String) -> Vec<Vec<i32>> {
    let mut pages: Vec<Vec<i32>> = Vec::new();
    for line in file.lines() {
        if line.contains(',') {
            let vals: Vec<i32> = line
                .split(',')
                .filter_map(|s| s.trim().parse::<i32>().ok())
                .collect();
            pages.push(vals);
        }
    }
    pages
}

fn main() {
    //let sample_rules = vec![(47,53), (47,61), (47,75)];
    //let sample_pages = vec![75,47,61,53,29];
    let file_content = fs::read_to_string("./inps/5.txt").unwrap();
    let file_pages_content = file_content.clone();

    let rules: Vec<(i32, i32)> = process_orders(file_content);
    let pages: Vec<Vec<i32>> = process_pages(file_pages_content);
    //let mut ans: bool = false;

    //let mut cnt = 0;
    let mut count_these: Vec<i32> = Vec::new();

    for line in pages {
        if is_ready(&rules, &line) {
            //cnt += 1;
            let mid = line.len() / 2;
            let temp_val = line[mid];
            count_these.push(temp_val);
        }
    }
    let final_ans: i32 = count_these.into_iter().fold(0, |acc, x| acc + x);
    println!("{final_ans}");
}
