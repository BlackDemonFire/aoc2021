use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let lines = contents.split("\n");
    let mut fwd1 = u16::MIN;
    let mut down = u16::MIN;
    let mut up = u16::MIN;
    let mut fwd2: i64 = 0;
    let mut alt: i64 = 0;
    let mut aim: i64 = 0;
    for line in lines {
        let mut it = line.split_whitespace();
        let direction = it.next().unwrap();
        let amount = it.next().unwrap().parse::<u16>().unwrap();
        match direction {
            "forward" => {
                fwd1 += amount;
                fwd2 += amount as i64;
                alt += aim * amount as i64;
            },
            "up" => {
                up += amount;
                aim -= amount as i64;
            },
            "down" => {
                down += amount;
                aim += amount as i64;
            },
            _ => println!("Strange input: {} {}", direction, amount),
        }
    }
    println!("task 1: {}", fwd1 as u32 * (down - up) as u32);
    println!("task 2: {}", fwd2 * alt)
}
