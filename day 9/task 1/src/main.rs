use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("input").unwrap();
    let lines = contents.split("\n");
    let field = lines
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();
    let mut result = u32::MIN;
    for (line_index, row) in field.iter().enumerate() {
        for (column, &cell) in row.iter().enumerate() {
            if line_index == 0 {
                if column == 0 {
                    if cell < row[column + 1] && cell < field[line_index + 1][column] {
                        result += cell + 1;
                    }
                } else if column + 1 == row.len() {
                    if cell < row[column - 1] && cell < field[line_index + 1][column] {
                        result += cell + 1;
                    }
                } else {
                    if cell < row[column - 1]
                        && cell < row[column + 1]
                        && cell < field[line_index + 1][column]
                    {
                        result += cell + 1;
                    }
                }
            } else if line_index + 1 == field.len() {
                if column == 0 {
                    if cell < row[column + 1] && cell < field[line_index - 1][column] {
                        result += cell + 1;
                    }
                } else if column + 1 == row.len() {
                    if cell < row[column - 1] && cell < field[line_index - 1][column] {
                        result += cell + 1;
                    }
                } else {
                    if cell < row[column - 1]
                        && cell < row[column + 1]
                        && cell < field[line_index - 1][column]
                    {
                        result += cell + 1;
                    }
                }
            } else {
                if column == 0 {
                    if cell < row[column + 1] && cell < field[line_index - 1][column] && cell < field[line_index + 1][column] {
                        result += cell + 1;
                    }
                } else if column + 1 == row.len() {
                    if cell < row[column - 1] && cell < field[line_index - 1][column] && cell < field[line_index + 1][column] {
                        result += cell + 1;
                    }
                } else {
                    if cell < row[column - 1]
                        && cell < row[column + 1]
                        && cell < field[line_index - 1][column]
                        && cell < field[line_index + 1][column]
                    {
                        result += cell + 1;
                    }
                }
            }
        }
    }
    println!("{}", field.len()*field[0].len());
    println!("{}", result)
}
