use std::fs;

fn main() {
    println!("Hello, world!");
    let contents: String = fs::read_to_string("input.txt").expect("Error in reading file");
    let split = contents.split_whitespace();

    let mut v: Vec<i32> = Vec::new();
    for s in split {
        let num: i32 = match s.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Error parsing string {s}");
                9999 // TODO improve error handling
            },
        };
        v.push(num);
    }

    let l = v.len();
    let mut i = 0;
    let mut count = 0;
    while i < l -3 {
        let a = v[i] + v[i+1] + v[i+2];
        let b = v[i+1] + v[i+2] + v[i+3];
        if a < b {
            count += 1;
        }
        i += 1;
    }

    println!("Count: {count}");
}
