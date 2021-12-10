use std::fs;

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let lines = contents.split("\n");
    let mut fwd = u16::MIN;
    let mut down = u16::MIN;
    let mut up = u16::MIN;
    for line in lines {
        let mut it = line.split_whitespace();
        let direction = it.next().unwrap();
        let amount = it.next().unwrap().parse::<u16>().unwrap();
        match direction {
            "forward" => fwd += amount,
            "up" => up += amount,
            "down" => down += amount,
            _ => println!("Strange input: {} {}", direction, amount),
        }
    }
    println!("{}", fwd as u32 * (down - up) as u32);
}
