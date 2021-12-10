use std::fs;

fn main() {
    let filename = "input";
    let contents = fs::read_to_string(filename).unwrap();
    let lines = contents.split_whitespace();
    let mut t1_last = u16::MAX;
    let mut count = u16::MIN;
    let mut sum = u16::MIN;
    let mut t2_lines = lines.clone();
    let mut prelast = t2_lines.next().unwrap().parse::<u16>().unwrap();
    let mut last = t2_lines.next().unwrap().parse::<u16>().unwrap();
    let mut last_sum = u16::MAX;
    for cur in lines.clone() {
        let num = cur.parse::<u16>().unwrap();
        if num > t1_last {
            count += 1;
        }
        t1_last = num;
    }
    println!("task 1: {}", count);
    for cur in t2_lines {
        let num = cur.parse::<u16>().unwrap();
        if num + last + prelast > last_sum {
            sum += 1;
        }
        last_sum = num + last + prelast;
        prelast = last;
        last = num;
    }
    println!("task 2: {}", sum);
}
