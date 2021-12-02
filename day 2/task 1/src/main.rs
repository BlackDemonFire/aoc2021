use std::fs;

fn main() {
    let contents = fs::read_to_string("input")
        .expect("Something went wrong reading the file");
    let mut lines = contents.split("\n");
    let mut fwd = u16::MIN;
    let mut down = u16::MIN;
    let mut up = u16::MIN;
    loop {
        let cur = lines.next();
        if cur == None {
            break;
        };
        let line = cur.unwrap();
        let mut it = line.split_whitespace();
        let direction = it.next().unwrap();
        let amount = u16::from_str_radix(it.next().unwrap(), 10).unwrap();
        match direction {
            "forward" => fwd += amount,
            "up" => up += amount,
            "down" => down += amount,
            _ => println!("Strange input: {} {}", direction, amount)
        }
    }
    println!("{}", fwd as u32 * (down - up) as u32);
}
