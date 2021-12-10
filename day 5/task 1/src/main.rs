use std::{cmp, fs};

struct Line {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
}

fn main() {
    let contents = fs::read_to_string("input").unwrap();
    let mut field = vec![vec![u8::MIN; 1000]; 1000];
    let lines = contents
        .lines()
        .map(|l| {
            let (pa, pb) = l.split_once(" -> ").unwrap();
            let (xa, ya) = pa.split_once(",").unwrap();
            let (xb, yb) = pb.split_once(",").unwrap();
            Line {
                x1: xa.parse::<u16>().unwrap(),
                x2: xb.parse::<u16>().unwrap(),
                y1: ya.parse::<u16>().unwrap(),
                y2: yb.parse::<u16>().unwrap(),
            }
        })
        .filter(|l| l.x1 == l.x2 || l.y1 == l.y2);

    for line in lines {
        if line.x1 == line.x2 {
            for y in cmp::min(line.y1, line.y2)..=cmp::max(line.y1, line.y2) {
                field[line.x1 as usize][y as usize] += 1;
            }
        } else {
            for x in cmp::min(line.x1, line.x2)..=cmp::max(line.x1, line.x2) {
                field[x as usize][line.y1 as usize] += 1;
            }
        }
    }
    let mut result = 0;
    for cell in field.iter().flatten() {
        if cell > &1 {
            result += 1;
        }
    }
    println!("{}", result)
}
