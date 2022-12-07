use std::fs;

fn main() {
    let contents: String = fs::read_to_string("./src/input.txt").expect("Error in reading file");
    let split: Vec<&str> = contents.split("\n").collect();
    // println!("{:?}", split);
    let mut max_cals = 0;
    let mut curr_cals = 0;

    for s in split {
        if s == "" {
            curr_cals = 0;
            continue;
        }
        let num: i32 = match s.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Error parsing string {s}");
                -1 // TODO improve error handling
            },
        };
        curr_cals += num;
        if curr_cals > max_cals {
            max_cals = curr_cals;
        }
    }

    println!("Max calories: {max_cals}");
}
