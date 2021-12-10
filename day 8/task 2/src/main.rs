use std::fs::read_to_string;

fn main() {
    let raw_input = read_to_string("./input").unwrap();
    let input = raw_input
        .lines()
        .map(|x| {
            let (unique, num) = x.split_once(" | ").unwrap();
            return (
                unique.split(" ").collect::<Vec<_>>(),
                num.split(" ").collect::<Vec<_>>(),
            );
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (unique, num) in input.iter() {
        sum += solve(unique, num);
    }
    println!("{}", sum);
}

fn sortstr(letters: &str) -> String {
    let mut chars = letters.chars().collect::<Vec<_>>();
    chars.sort_by(|a, b| a.cmp(b));
    return String::from_iter(chars);
}

fn solve(unique: &Vec<&str>, num: &Vec<&str>) -> u32 {
    // Btw this is a horrible solution and I hate it

    let digit_one = unique.iter().find(|x| x.len() == 2).unwrap();
    let digit_four = unique.iter().find(|x| x.len() == 4).unwrap();
    let digit_seven = unique.iter().find(|x| x.len() == 3).unwrap();
    let digit_eight = unique.iter().find(|x| x.len() == 7).unwrap();

    let top_char = digit_seven
        .chars()
        .find(|x| !digit_one.contains(*x))
        .unwrap();
    let middle_char = digit_four
        .chars()
        .filter(|x| !digit_one.contains(*x))
        .find(|x| {
            unique
                .iter()
                .filter(|x2| x2.contains(*x))
                .collect::<Vec<_>>()
                .len()
                == 7
        })
        .unwrap();
    let bottom_char = unique
        .iter()
        .find(|x| {
            x.len() == 5
                && x.contains(top_char)
                && x.contains(middle_char)
                && digit_one.chars().all(|c| x.contains(c))
        })
        .unwrap()
        .chars()
        .find(|&x| x != top_char && x != middle_char && digit_one.chars().all(|c| x != c))
        .unwrap();

    let left_chars = digit_eight
        .chars()
        .filter(|&x| {
            x != top_char
                && x != middle_char
                && x != bottom_char
                && digit_one.chars().all(|c| x != c)
        })
        .collect::<Vec<_>>();

    let left_top_char = digit_four
        .chars()
        .find(|&x| x != middle_char && digit_one.chars().all(|c| x != c))
        .unwrap();

    let left_bottom_char = *left_chars.iter().find(|&&x| x != left_top_char).unwrap();

    let right_top_char = unique
        .iter()
        .find(|x| x.len() == 5 && x.contains(left_bottom_char))
        .unwrap()
        .chars()
        .find(|&x| x != top_char && x != middle_char && x != bottom_char && x != left_bottom_char)
        .unwrap();

    let right_bottom_char = digit_one.chars().find(|&x| x != right_top_char).unwrap();

    let zero = sortstr(&format!(
        "{}{}{}{}{}{}",
        top_char, bottom_char, left_top_char, left_bottom_char, right_bottom_char, right_top_char
    ));
    let szero = zero.as_str();
    let one = sortstr(&format!("{}{}", right_top_char, right_bottom_char));
    let sone = one.as_str();
    let two = sortstr(&format!(
        "{}{}{}{}{}",
        top_char, middle_char, bottom_char, right_top_char, left_bottom_char
    ));
    let stwo = two.as_str();
    let three = sortstr(&format!(
        "{}{}{}{}{}",
        top_char, middle_char, bottom_char, right_top_char, right_bottom_char
    ));
    let sthree = three.as_str();
    let four = sortstr(&format!(
        "{}{}{}{}",
        left_top_char, middle_char, right_top_char, right_bottom_char
    ));
    let sfour = four.as_str();
    let five = sortstr(&format!(
        "{}{}{}{}{}",
        top_char, middle_char, bottom_char, left_top_char, right_bottom_char
    ));
    let sfive = five.as_str();
    let six = sortstr(&format!(
        "{}{}{}{}{}{}",
        top_char, middle_char, bottom_char, left_top_char, left_bottom_char, right_bottom_char
    ));
    let ssix = six.as_str();
    let seven = sortstr(&format!(
        "{}{}{}",
        top_char, right_top_char, right_bottom_char
    ));
    let sseven = seven.as_str();
    let eight = sortstr(&format!(
        "{}{}{}{}{}{}{}",
        top_char,
        middle_char,
        bottom_char,
        left_top_char,
        left_bottom_char,
        right_bottom_char,
        right_top_char
    ));
    let seight = eight.as_str();
    let nine = sortstr(&format!(
        "{}{}{}{}{}{}",
        top_char, middle_char, bottom_char, left_top_char, right_bottom_char, right_top_char
    ));
    let snine = nine.as_str();

    let mut sum = 0;
    for (i, n) in num.iter().rev().enumerate() {
        let nn = sortstr(n);
        sum += match nn {
            _ if nn.as_str() == szero => 0 * (10u32).pow(i as u32),
            _ if nn.as_str() == sone => 1 * (10u32).pow(i as u32),
            _ if nn.as_str() == stwo => 2 * (10u32).pow(i as u32),
            _ if nn.as_str() == sthree => 3 * (10u32).pow(i as u32),
            _ if nn.as_str() == sfour => 4 * (10u32).pow(i as u32),
            _ if nn.as_str() == sfive => 5 * (10u32).pow(i as u32),
            _ if nn.as_str() == ssix => 6 * (10u32).pow(i as u32),
            _ if nn.as_str() == sseven => 7 * (10u32).pow(i as u32),
            _ if nn.as_str() == seight => 8 * (10u32).pow(i as u32),
            _ if nn.as_str() == snine => 9 * (10u32).pow(i as u32),
            _ => 0,
        }
    }
    return sum;
}
