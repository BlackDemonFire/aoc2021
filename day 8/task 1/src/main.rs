use std::fs::read_to_string;

fn main() {
    let content = read_to_string("input").unwrap();
    let outputs = content.split("\n").map(|f| f.split_once(" | ").unwrap().1);
    let digits = outputs.map(|e| e.split_whitespace()).flatten();
    let mut count = u32::MIN;
    for digit in digits {
        if [2, 3, 4, 7].contains(&digit.len()) {
            count += 1;
        }
    }
    println!("{}", count)
}
