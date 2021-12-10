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

    let mut used_nums: Vec<u8> = Vec::new();
    for num in nums.into_iter() {
        used_nums.push(num);
        let finished = &fields
            .clone()
            .into_iter()
            .filter(|field| check_line(&field.to_vec(), &used_nums))
            .collect::<Vec<Vec<u8>>>();
        if finished.len() > 0 {
            for f in finished.iter() {
                let sum = calc_sum(f, &used_nums);
                println!("{}", sum);
            }
            std::process::exit(0);
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

fn calc_sum(board: &Vec<u8>, used_nums: &Vec<u8>) -> u32 {
    let mut marked = 0;
    let mut unmarked = 0;
    let mut cells = Vec::<u8>::new();
    board.into_iter().for_each(|cell| {
        if !cells.contains(&cell) {
            if used_nums.contains(&cell) {
                println!("{}\tmarked", cell);
                marked += *cell as u32;
            } else {
                println!("{}\tunmarked", cell);
                unmarked += *cell as u32;
            }
            cells.push(*cell);
        }
    });
    print!(
        "\nused {} nums\n{} * {} = ",
        used_nums.len(),
        unmarked,
        used_nums.last().unwrap()
    );
    return unmarked * *used_nums.last().unwrap() as u32;
}
