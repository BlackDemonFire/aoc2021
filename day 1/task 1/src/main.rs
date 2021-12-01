use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    let mut lines = contents.split_whitespace();
    let mut last = u16::MAX;
    let mut count = u16::MIN;
    loop {
        let cur = lines.next();
        if cur == None {
            break;
        };
        let num = u16::from_str_radix(cur.unwrap(), 10).unwrap();
        if num > last {
            count += 1;
        }
        last = num;
    }
    println!("{}", count);
}
