use std::fs;
use regex::Regex;

#[allow(dead_code)]
pub fn s1() -> i64 {
    let mut grand_total: i64 = 0;


    let num_re = Regex::new(r"\d+").unwrap();

    for ln_raw in fs::read_to_string("data/d4.input").unwrap().lines() {
        let mut subtotal = 0;

        let splits = ln_raw.split(": ").collect::<Vec<_>>()[1].split(" | ").collect::<Vec<_>>();
        let winning_input: Vec<&str> = num_re.find_iter(splits[0]).map(|x| x.as_str()).collect();
        let my_input: Vec<&str> = num_re.find_iter(splits[1]).map(|x| x.as_str()).collect();

        for wi in my_input {
            if winning_input.contains(&wi) {
                if subtotal == 0 {
                    subtotal += 1;
                } else {
                    subtotal *= 2;
                }
            }
        }

        grand_total += subtotal;
    }

    return grand_total
}


#[allow(dead_code)]
pub fn s2() -> i64 {

    let num_re = Regex::new(r"\d+").unwrap();

    let mut card_counts: Vec<i64> = vec![];

    let mut winning_inputs: Vec<Vec<String>> = vec![];
    let mut my_inputs: Vec<Vec<String>> = vec![];


    for ln_raw in fs::read_to_string("data/d4.input").unwrap().lines() {

        let all_splits = ln_raw.split(": ").collect::<Vec<_>>();
        card_counts.push(1);


        let input_splits = all_splits[1].split(" | ").collect::<Vec<_>>();
        winning_inputs.push(
            num_re.find_iter(input_splits[0]).map(|x| x.as_str().to_owned() ).collect()
        );

        my_inputs.push(
            num_re.find_iter(input_splits[1]).map(|x| x.as_str().to_owned()).collect()
        );
    }

    for i in 0..card_counts.len() {

        let mut subtotal = 0;
        for my_in in &my_inputs[i] {
            if winning_inputs[i].contains(my_in) {
                subtotal += 1;
            }
        }
        
        if subtotal > 0 {
            
            for j in 0..subtotal {
                card_counts[i+j+1] += 1*card_counts[i];
            }
        }
    }

    return card_counts.into_iter().sum();
}

