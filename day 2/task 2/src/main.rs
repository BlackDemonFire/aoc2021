use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let lines = contents.split("\n");
    let mut fwd: i64 = 0;
    let mut alt: i64 = 0;
    let mut aim: i64 = 0;
    for line in lines {
        let mut it = line.split_whitespace();
        let direction = it.next().unwrap();
        let amount = it.next().unwrap().parse::<i64>().unwrap();
        match direction {
            "forward" => {
                fwd += amount;
                alt += aim * amount;
            }
            "up" => aim -= amount,
            "down" => aim += amount,
            _ => println!("Strange input: {} {}", direction, amount),
        }
    }
    println!("{}", fwd * alt);
}
