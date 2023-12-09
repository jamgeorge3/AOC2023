use std::fs;
use regex::Regex;

const TEST_INPUT: &str = 
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";


fn get_input(test:bool) -> Vec<Vec<i64>> {
    let num_re: Regex = Regex::new(r"-?\d+").unwrap();

    let test_input = TEST_INPUT.to_owned();
    let file_input = fs::read_to_string("data/d9.input")
        .unwrap().to_owned();

    let input_to_use = if test { test_input } else { file_input };

    let num_arrays: Vec<Vec<i64>> = input_to_use.lines()
        .map(|x| 
            num_re.find_iter(x).map(|y| 
                y.as_str().to_owned().parse::<i64>().unwrap())
            .collect()
        ).collect();

    return num_arrays
}


fn get_subtree(
    row: Vec<i64>, all_rows: &mut Vec<Vec<i64>>, depth: &mut i32
) -> () {

    let mut next_row: Vec<i64> = vec![];
    for i in 0..row.len()-1 {
        next_row.push(row[i+1] - row[i]);
    }

    all_rows.push(next_row.clone());

    // Break if all zeroes
    if !next_row.iter().all(|x| *x==0) {
        *depth += 1;
        get_subtree(next_row, all_rows, depth);
    }
}


#[allow(dead_code)]
pub fn s1_2(prepend: bool) -> i64 {
    let oasis_rows = get_input(false);

    let mut depth = 0;
    let mut total = 0;
    for row in oasis_rows {
        let mut row_add = row.clone();
        if prepend {
            row_add.reverse();
        }

        let mut tree: Vec<Vec<i64>> = Vec::from([row_add.clone()]);
        get_subtree(row_add, &mut tree, &mut depth);

        // Calculate next
        let mut i = tree.len()-1;
        let mut next_in_sequence = 0;

        while i != 0 {
            let add_val = next_in_sequence + tree[i-1].last().unwrap();
            next_in_sequence = add_val;
            i -= 1;
        }
        total += next_in_sequence;
    }

    return total;
}
