use std::fs;

fn main() {
    let contents: String = fs::read_to_string("./src/input.txt").expect("Error in reading file");
    let split: Vec<&str> = contents.split("\n").collect();
    // println!("{:?}", split);
    let mut curr_cals = 0;
    let mut all_calories: Vec<i32> = Vec::new();

    for s in split {
        if s == "" {
            all_calories.push(curr_cals);
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
    }

    all_calories.sort();
    all_calories.reverse();
    let total_cals = all_calories[0] + all_calories[1] + all_calories[2]; // Could this be a slice?
    println!("Total calories of top 3: {total_cals}");

}
