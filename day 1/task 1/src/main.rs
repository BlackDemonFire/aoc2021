use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let lines = contents.split_whitespace();
    let mut last = u16::MAX;
    let mut count = u16::MIN;
    for cur in lines {
        let num = cur.parse::<u16>().unwrap();
        if num > last {
            count += 1;
        }
        last = num;
    }
    println!("task 1: {}", count);
}
