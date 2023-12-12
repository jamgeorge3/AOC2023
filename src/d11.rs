use std::fs;
use regex::Regex;
use std::collections::HashSet;
use std::cmp::max;
use std::cmp::min;

const TEST_INPUT: &str = 
"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";


fn get_input(test:bool) -> (Vec<[i32; 2]>, HashSet<usize>, HashSet<usize>) {
    let star_re: Regex = Regex::new(r"#").unwrap();

    let test_input = TEST_INPUT.to_owned();
    let file_input = fs::read_to_string("data/d11.input")
        .unwrap().to_owned();

    let input_to_use = if test { test_input } else { file_input };

    let mut rows_seen = HashSet::new();
    let mut cols_seen = HashSet::new();

    let stars: Vec<[i32; 2]> = input_to_use.lines().enumerate()
        .map(| (row_i, row) | 
            star_re.find_iter(row).map(|m| { 
                let col_i: usize = m.start(); 
                rows_seen.insert(row_i);
                cols_seen.insert(col_i);
                [row_i as i32, col_i as i32] 
            }).collect::<Vec<[i32; 2]>>() 
        ).flatten().collect::<Vec<[i32; 2]>>();

    let col_max = cols_seen.iter().max().unwrap().to_owned();
    let col_min = cols_seen.iter().min().unwrap().to_owned();

    let row_max = rows_seen.iter().max().unwrap().to_owned();
    let row_min = rows_seen.iter().min().unwrap().to_owned();

    let mut empty_cols = HashSet::new();
    let mut empty_rows = HashSet::new();

    for col in col_min..=col_max {
        if !cols_seen.contains(&col) {
            empty_cols.insert(col);
        }
    }

    for row in row_min..=row_max {
        if !rows_seen.contains(&row) {
            empty_rows.insert(row);
        }
    }
    
    return (stars, empty_rows, empty_cols);
}


#[allow(dead_code)]
pub fn s1_2(multiplier: i64) -> i64 {

    let (stars, empty_rows, empty_cols) = get_input(false);

    let mut total_distance = 0;
    for i in 0..stars.len() {
        for j in i+1..stars.len() {

            let x_lg = max(stars[i][0], stars[j][0]);
            let x_sm = min(stars[i][0], stars[j][0]);

            let y_lg = max(stars[i][1], stars[j][1]);
            let y_sm = min(stars[i][1], stars[j][1]);

            total_distance += (x_lg - x_sm) as i64 + (y_lg - y_sm) as i64;

            for (empty_x, sm, lg) in 
                [(&empty_rows, x_sm, x_lg), (&empty_cols, y_sm, y_lg)] 
            {
                total_distance += empty_x.iter()
                    .map(|row| { 
                        let i = if (sm..=lg).contains(&(*row as i32)) {(1*multiplier)-1} else {0}; i 
                    }).sum::<i64>();
            }
        }
    }

    return total_distance;
}

