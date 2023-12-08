use std::fs;
use std::collections::HashMap;
use std::iter::zip;
use regex::Regex;
use super::tools::lcm;

// const TEST_INPUT: &str = 
// "LLR

// AAA = (BBB, BBB)
// BBB = (AAA, ZZZ)
// ZZZ = (ZZZ, ZZZ)";

const TEST_INPUT: &str = 
"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";


fn get_input(test:bool) -> (Vec<usize>, HashMap<String, [String; 2]>) {
    let coord_re: Regex = Regex::new(r"[A-Z0-9]{3}").unwrap();

    let test_input = TEST_INPUT.to_owned();
    let file_input = fs::read_to_string("data/d8.input")
        .unwrap().to_owned();

    let input_to_use = if test { test_input } else { file_input }; 

    // println!("IN {input_to_use:?}");
    
    let mut keys: Vec<String> = vec![];
    let mut vals: Vec<[String; 2]> = vec![];

    let input_parts = input_to_use.split("\n\n").map(String::from).collect::<Vec<_>>();

    let directions: Vec<usize> = input_parts[0].replace('L', "0").replace('R', "1")
        .chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>();

    for line in input_parts[1].lines() {
        let stuff: Vec<String> = coord_re.find_iter(line)
            .map(|x| x.as_str().to_owned()).collect();

        keys.push(stuff[0].clone());
        vals.push([stuff[1].clone(), stuff[2].clone()]);
    }

    let map = zip(keys, vals)
        .collect::<HashMap<String, [String; 2]>>();
    
    return (directions, map);
}


#[allow(dead_code)]
pub fn s1() -> u64 {
    let mut steps_taken: u64 = 0;

    let finish = "ZZZ";
    let mut curr_loc = "AAA";

    let (directions, map) = get_input(false);

    // println!("Directions {:?}, Map {:?}", directions, map);

    while curr_loc != finish {
        for d in &directions {
            steps_taken += 1;
            // println!("CURR LOC {curr_loc}");
            curr_loc = &map.get(curr_loc).unwrap()[*d];
            if curr_loc == finish {
                break;
            }
        }
    }

    return steps_taken;
}


#[allow(dead_code)]
pub fn s2() -> u64 {
    let (directions, map) = get_input(false);

    let mut starting_locs: Vec<String> = map.keys().map(String::from).collect::<Vec<String>>();
    starting_locs.retain(|x| x.as_bytes()[2] as char == 'A');

    let mut all_best_routes: Vec<i32> = vec![];
    for starting_loc in starting_locs {
        let mut curr_loc = &starting_loc.clone();

        let mut steps_taken = 0;

        while (curr_loc.as_bytes()[2]) as char != 'Z' {
            for d in &directions {
                steps_taken += 1;

                curr_loc = &map.get(curr_loc).unwrap()[*d];
                if (curr_loc.as_bytes()[2]) as char == 'Z' {
                    break;
                }
            }
        }

        all_best_routes.push(steps_taken);
    }

    let mut lcm_so_far: usize = all_best_routes[0] as usize;
    for i in all_best_routes[1..].iter() {
        let thing = lcm::lcm(lcm_so_far as usize, *i as usize);
        lcm_so_far = thing.clone();
    }

    return lcm_so_far as u64;
}
