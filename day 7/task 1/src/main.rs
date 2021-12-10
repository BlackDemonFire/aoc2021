use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("input").unwrap();
    let positions = contents.split(",").map(|pos| pos.parse::<i64>().unwrap());
    let mut sum = u64::MAX;
    for cur in positions.clone().min().unwrap()..=positions.clone().max().unwrap() {
        let mut lsum = u64::MIN;
        for pos in positions.clone() {
            lsum += (pos - cur).abs() as u64;
        }
        if lsum < sum {
            sum = lsum
        }
    }
    println!("{}", sum);
}
