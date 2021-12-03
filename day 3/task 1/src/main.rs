use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let mut lines = contents.split_whitespace();
    let mut ones = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut zeroes = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let mut gamma = String::with_capacity(12);
    let mut epsilon = String::with_capacity(12);
    loop {
        let cur = lines.next();
        if cur == None {
            break;
        };
        let chars = cur.unwrap().chars();
        let mut i = 0;
        for c in chars {
            match c {
                '0' => zeroes[i] += 1,
                '1' => ones[i] += 1,
                _ => println!("Unknown char."),
            }
            i += 1;
        }
    }
    let r = std::ops::Range { start: 0, end: 12 };
    for i in r {
        if ones[i] >= zeroes[i] {
            gamma += "1";
            epsilon += "0";
        } else {
            gamma += "0";
            epsilon += "1";
        }
    }
    let e = u32::from_str_radix(epsilon.as_str(), 2).unwrap();
    let g = u32::from_str_radix(gamma.as_str(), 2).unwrap();
    println!("{}", e * g);
}
