use std::fs;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn s1() -> u64 {
    let mut grand_total: u64 = 0;

    for line in fs::read_to_string("data/d1.input").unwrap().lines() {
        let mut ints_in_line: Vec<u32> = vec![]; 

        for c in line.chars() {
            let intchar: Option<u32> = c.to_digit(10);
            if intchar.is_some() {
                ints_in_line.push(intchar.unwrap())
            }
        }

        if ints_in_line.len() > 0 {
            let tot = format!(
                "{}{}", 
                ints_in_line.first().unwrap().to_string(), 
                ints_in_line.last().unwrap().to_string()
            );
            grand_total += tot.parse::<u64>().unwrap();
        }
    }

    return grand_total;
}


#[allow(dead_code)]
pub fn s2() -> u64 {
    let word_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut grand_total: u64 = 0;

    for line in fs::read_to_string("data/d1.input").unwrap().lines() {
        let mut ints_in_line = vec![]; 

        for (i, c) in line.chars().enumerate() {
            let intchar: Option<u32> = c.to_digit(10);
            if intchar.is_some() {
                ints_in_line.push((i, intchar.unwrap()))
            }
        }

        for k in word_map.keys() {
            let found: Vec<_> = line.match_indices(k).collect();
            for (i, v) in found {
                ints_in_line.push(
                    (i, *word_map.get(v).unwrap())
                )
            }
        }

        ints_in_line.sort_by(
            |a, b| a.partial_cmp(b).unwrap()
        );

        if ints_in_line.len() > 0 {

            let tot = format!(
                "{}{}", 
                ints_in_line.first().unwrap().1, 
                ints_in_line.last().unwrap().1
            );
            grand_total += tot.parse::<u64>().unwrap();
        }
    }

    return grand_total;
}

