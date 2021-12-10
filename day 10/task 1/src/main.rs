use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("input").unwrap();
    let lines = contents.split("\n");
    let mut result = u32::MIN;
    for line in lines {
        let chars = line.chars();
        let mut checked = Vec::<char>::new();
        for char in chars {
            if ['(', '[', '{', '<'].contains(&char) {
                checked.push(char);
            } else {
                let top = checked.pop().unwrap();
                match char {
                    ')' => {
                        if top != '(' {
                            result += 3;
                            break;
                        } 
                    }
                    ']' => {
                        if top != '[' {
                            result += 57;
                            break;
                        } 
                    }
                    '}' => {
                        if top != '{' {
                            result += 1197;
                            break;
                        } 
                    }
                    '>' => {
                        if top != '<' {
                            result += 25137;
                            break;
                        } 
                    }
                    _ => {
                        eprintln!("unknown char: {}", char)
                    }
                }
            }
        }
    }
    println!("task 1: {}", result)
}