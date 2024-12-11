use std::fs;
fn is_ready(ordering_rules: &[(i32,i32)], page_nums: &[i32]) -> bool {
    for (a,b) in ordering_rules.iter() { 
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
            (None, None) => return true,
            (None, Some(_second)) => return true,
            (Some(_first), None) => return true,
            _ => return false,
        }
    }
    true
}

fn process_orders(file: String) -> Vec<(i32,i32)> {
    let mut orders: Vec<(i32,i32)> = Vec::new();
    for line in file.lines() {
        if let Some(pos) = line.find('|') {
            let (left, right) = line.split_at(pos);

            if let (Ok(a), Ok(b)) = (left.parse::<i32>(), right[1..].parse::<i32>()) {
                orders.push((a,b));
            }
        }
    }
    orders
}

fn process_pages(file: String) -> Vec<Vec<i32>> {
    let mut pages: Vec<Vec<i32>> = Vec::new();
    for line in file.lines() { 
        if line.contains(',') {
            let vals: Vec<i32> = line.split(',')
                                    .filter_map(|s| s.trim().parse::<i32>().ok())
                                    .collect();
            pages.push(vals);
        }
    }
    pages
}

fn main () { 
    //let sample_rules = vec![(47,53), (47,61), (47,75)];
    //let sample_pages = vec![75,47,61,53,29];
    let file_content = fs::read_to_string("./inps/5.txt").unwrap();
    let file_pages_content = file_content.clone();
    let rules: Vec<(i32,i32)> = process_orders(file_content);
    let pages: Vec<Vec<i32>> = process_pages(file_pages_content);
    //let mut ans: bool = false;
    let mut cnt = 0;
    let mut count_these: Vec<Vec<i32>> = Vec::new();
    for line in pages {
        let mut ans: bool = false;
        ans = is_ready(&rules, &line); // is_ready takes vec<(i32,i32)> and &vec<i32> 
        if ans {
            cnt += 1;
            count_these.push(line);
        }
    }
    //println!("{cnt}");
    let mut cum_sum: i32 = 0;
    for each in &count_these {
        println!("{:?}", each);
        let mid: usize = each.len()/2; 
        let temp_val = each[mid];
        println!("temp_val is: {}", temp_val);
        cum_sum += temp_val;
        println!("running sum is up to: {}", cum_sum);
    }
    //println!("{ans}");
    println!("Cumulative sum is: {}", cum_sum);// answer is currently too high
}

