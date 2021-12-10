use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("input").unwrap();
    let lines = contents.split("\n");
    let mut scores = Vec::<u64>::new();
    for line in lines {
        let chars = line.chars();
        let mut checked = Vec::<char>::new();
        let mut corrupted = false;
        let mut score = 0;
        for char in chars {
            if ['(', '[', '{', '<'].contains(&char) {
                checked.push(char);
            } else {
                let top = checked.pop().unwrap();
                match char {
                    ')' => {
                        if top != '(' {
                            corrupted = true;
                            break;
                        }
                    }
                    ']' => {
                        if top != '[' {
                            corrupted = true;
                            break;
                        }
                    }
                    '}' => {
                        if top != '{' {
                            corrupted = true;
                            break;
                        }
                    }
                    '>' => {
                        if top != '<' {
                            corrupted = true;
                            break;
                        }
                    }
                    _ => {
                        eprintln!("unknown char: {}", char)
                    }
                }
            }
        }
        if corrupted {
            continue;
        }
        for char in checked.iter().rev() {
            score *= 5;
            score += match char {
                '(' => 1,
                '[' => 2,
                '{' => 3,
                '<' => 4,
                _ => 0,
            }
        }
        scores.push(score);
    }
    scores.sort();
    scores.dedup();
    println!("task 2: {}", scores[scores.len() / 2]);
}
