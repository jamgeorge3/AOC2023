use std::fs;
use regex::Regex;

struct Coordinate {
    row: i32,
    col_start: i32,
    col_end: i32
}

fn get_coordinates(symbol_char: Option<char>) -> (Vec<Coordinate>, Vec<(i32, Coordinate)>) {
    // (row, column)
    let mut symbol_coords: Vec<Coordinate> = vec![];

    // (val, row, num_start_col, num_end_col)
    let mut num_coords: Vec<(i32, Coordinate)> = vec![];

    
    let num_re = Regex::new(r"\d+").unwrap();

    // Get symbol and number coordinates
    for (row, line) in fs::read_to_string("data/d3.input").unwrap().lines().enumerate() {
        for num_match in num_re.find_iter(line) {
            num_coords.push((
                num_match.as_str().parse::<i32>().unwrap(),
                Coordinate {
                    row: row as i32, 
                    col_start: num_match.start() as i32,
                    col_end: num_match.end() as i32
                }
            ))
        }

        for (col, ch) in line.chars().enumerate() {
            if let Some(symbol) = symbol_char 
            {
                if !ch.is_ascii_digit() && ch == symbol {
                    symbol_coords.push(
                        Coordinate {
                            row: row as i32, 
                            col_start: col as i32,
                            col_end: col as i32
                        }
                    )    
                }
            } else if !ch.is_ascii_digit() && ch != '.' {
                symbol_coords.push(
                    Coordinate {
                        row: row as i32, 
                        col_start: col as i32,
                        col_end: col as i32
                    }
                )
            }
        }
    }

    return (symbol_coords, num_coords)
}


#[allow(dead_code)]
pub fn s1() -> i64 {
    let mut grand_total: i64 = 0;

    let (symbol_coords, num_coords) = get_coordinates(None);

    // Compare number locations to symbol locations to find out what is/isn't valid
    for n in &num_coords {
        // let col_range = n.2-1..n.3+1;
        let col_range = n.1.col_start-1..n.1.col_end+1;
        for s in &symbol_coords {

            // If row is one apart
            if (s.row - n.1.row).abs() < 2 && col_range.contains(&s.col_start) {
                grand_total += n.0 as i64;
                break;
            }
        }
    } 

    return grand_total
}


#[allow(dead_code)]
pub fn s2() -> i64 {
    let mut grand_total: i64 = 0;

    let (symbol_coords, num_coords) = get_coordinates(Some('*'));

    for s in &symbol_coords {

        let mut s_matched: Vec<i64> = vec![];

        for n in &num_coords {
            let col_range = n.1.col_start-1..n.1.col_end+1;
            // If row is one apart and within 1 col of the number
            if (s.row - n.1.row).abs() < 2 && col_range.contains(&s.col_start) {
                s_matched.push(n.0 as i64);
            }
        }

        if s_matched.len() == 2 {
            let gear_ratio: i64 = s_matched.iter().product();
            grand_total += gear_ratio;
        }
    }

    return grand_total
}

