use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let lines = contents.split_whitespace().collect::<Vec<&str>>();
    let co2lines = &lines;
    let o2lines = &lines;
    let co2index = 0;
    let o2index = 0;
    let co2count = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    let o2count = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];
    println!(
        "{}",
        u32::from_str_radix(check_o2(o2lines, o2index, o2count), 2).unwrap()
            * u32::from_str_radix(check_co2(co2lines, co2index, co2count), 2).unwrap()
    );
}

fn check_o2<'a>(o2lines: &Vec<&'a str>, o2index: usize, mut o2count: [i32; 12]) -> &'a str {
    for line in o2lines.iter() {
        match line.chars().nth(o2index).unwrap() {
            '0' => o2count[o2index] -= 1,
            '1' => o2count[o2index] += 1,
            _ => println!("Unknown char."),
        }
    }
    let c;
    if o2count[o2index] >= 0 {
        c = '1'
    } else {
        c = '0'
    }
    let oxlines = &o2lines
        .into_iter()
        .filter(|line| -> bool { line.chars().nth(o2index).unwrap() == c })
        .cloned()
        .collect::<Vec<&str>>();
    if oxlines.len() > 1 {
        return check_o2(oxlines, o2index + 1, o2count);
    } else {
        return oxlines[0];
    }
}

fn check_co2<'a>(co2lines: &Vec<&'a str>, co2index: usize, mut co2count: [i32; 12]) -> &'a str {
    for line in co2lines.iter() {
        match line.chars().nth(co2index).unwrap() {
            '0' => co2count[co2index] -= 1,
            '1' => co2count[co2index] += 1,
            _ => println!("Unknown char."),
        }
    }
    let c;
    if co2count[co2index] >= 0 {
        c = '0'
    } else {
        c = '1'
    }
    let oxlines = &co2lines
        .into_iter()
        .filter(|line| -> bool { line.chars().nth(co2index).unwrap() == c })
        .cloned()
        .collect::<Vec<&str>>();
    if oxlines.len() > 1 {
        return check_co2(oxlines, co2index + 1, co2count);
    } else {
        return oxlines[0];
    }
}
