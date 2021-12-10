use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let mut lines = contents.split_whitespace();
    let mut prelast = lines.next().unwrap().parse::<u16>().unwrap();
    let mut last = lines.next().unwrap().parse::<u16>().unwrap();
    let mut last_sum = u16::MAX;
    let mut sum = u16::MIN;
    for cur in lines {
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
