use std::fs;

fn main() {
    let contents = fs::read_to_string("input").expect("Something went wrong reading the file");
    let mut blocks = contents.split("\n\n");
    let nums: Vec<u8> = blocks
        .next()
        .unwrap()
        .split(",")
        .map(|elem| u8::from_str_radix(elem, 10).unwrap())
        .collect();
    let mut fields: Vec<Vec<u8>> = Vec::new();
    for block in blocks {
        let mut field: Vec<u8> = Vec::with_capacity(25);
        for elem in block.split_whitespace() {
            field.push(u8::from_str_radix(elem, 10).unwrap())
        }
        fields.push(field);
    }

    let field_iter = fields.iter();

    let mut used_nums: Vec<u8> = vec![];
    let mut winning_boards: Vec<usize> = vec![];
    let numbers = nums.iter();
    for n in numbers.clone() {
        used_nums.push(*n);
        for (i, board) in field_iter.clone().enumerate() {
            if !winning_boards.contains(&i) && check_line(&board, &used_nums) {
                if winning_boards.len() == field_iter.len() - 1 {
                    let mut sum = 0;
                    for c in board.iter() {
                        if !used_nums.contains(c) {
                            sum += *c as u16;
                        }
                    }
                    println!("{}", sum * *n as u16);
                    std::process::exit(0);
                }
                winning_boards.push(i);
            }
        }
    }
}

fn check_line(field: &Vec<u8>, used_nums: &Vec<u8>) -> bool {
    if (0..5).all(|num| used_nums.contains(&field[num])) {
        return true;
    }
    if (5..10).all(|num| used_nums.contains(&field[num])) {
        return true;
    }
    if (10..15).all(|num| used_nums.contains(&field[num])) {
        return true;
    }
    if (15..20).all(|num| used_nums.contains(&field[num])) {
        return true;
    }
    if (20..25).all(|num| used_nums.contains(&field[num])) {
        return true;
    }
    if (0..5).all(|num| used_nums.contains(&field[num * 5])) {
        return true;
    }
    if (0..5).all(|num| used_nums.contains(&field[num * 5 + 1])) {
        return true;
    }
    if (0..5).all(|num| used_nums.contains(&field[num * 5 + 2])) {
        return true;
    }
    if (0..5).all(|num| used_nums.contains(&field[num * 5 + 3])) {
        return true;
    }
    if (0..5).all(|num| used_nums.contains(&field[num * 5 + 4])) {
        return true;
    }
    if [0, 6, 12, 18, 24]
        .into_iter()
        .all(|num| used_nums.contains(&field[num]))
    {
        return true;
    }
    if [4, 8, 12, 16, 20]
        .into_iter()
        .all(|num| used_nums.contains(&field[num]))
    {
        return true;
    }
    return false;
}
