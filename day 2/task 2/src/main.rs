use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let mut lines = contents.split("\n");
    let mut fwd: i64 = 0;
    let mut alt: i64 = 0;
    let mut aim: i64 = 0;
    loop {
        let cur = lines.next();
        if cur == None {
            break;
        };
        let line = cur.unwrap();
        let mut it = line.split_whitespace();
        let direction = it.next().unwrap();
        let amount = i64::from_str_radix(it.next().unwrap(), 10).unwrap();
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
