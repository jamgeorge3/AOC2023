use std::fs;
use std::ops::Range;
use regex::Regex;

const TEST_INPUT: &str = 
"seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

#[derive(Debug)]
#[derive(Clone)]
struct MapPiece {
    range: Range<i64>,
    offset: i64
}


fn get_input(test:bool) -> (Vec<i64>, Vec<(String, Vec<MapPiece>)>) {

    let num_re: Regex = Regex::new(r"\d+").unwrap();

    let mut parts: Vec<String> = vec![];
    if test {
        parts = TEST_INPUT.split("\n\n").map(String::from).collect::<Vec<_>>();
    } else {
        parts = fs::read_to_string("data/d5.input")
            .unwrap().split("\r\n\r\n").map(String::from).collect::<Vec<_>>();
    }

    let seeds: Vec<i64> = num_re.find_iter(&parts[0])
        .map(|x| x.as_str().parse::<i64>().unwrap())
        .collect();

    let maps_raw: Vec<_> = parts[1..].iter()
        .map(|x| x.split("\n").collect::<Vec<_>>())
        .collect();

    let mut maps: Vec<(String, Vec<MapPiece>)> = vec![];
    for m_row in maps_raw {

        let map_pieces: Vec<MapPiece> = m_row[1..].to_owned().iter().map(|m| {
            let range_vals: Vec<i64> = num_re.find_iter(m)
                .map(|x| x.as_str().parse::<i64>().unwrap()).collect();
            MapPiece{
                range: range_vals[1]..(range_vals[1]+range_vals[2]), 
                offset: (range_vals[0] - range_vals[1])
            }
        }).collect();

        maps.push((m_row[0].to_owned(), map_pieces));
    } 

    // println!("{:?}", maps);
    return (seeds, maps);
}


#[allow(dead_code)]
pub fn s1() -> i64 {

    let (seeds, maps) = get_input(false);

    let lowest_vals: Vec<i64> = seeds.iter().map(|seed| {
        let mut next_check = seed.to_owned();
        for map_section in &maps {
            for m in &map_section.1 {
                if m.range.contains(&next_check) {
                    next_check = next_check+m.offset;
                    break;
                }
            }
        }
        next_check
    }).collect();

    return lowest_vals.iter().min().unwrap().to_owned();
}


#[allow(dead_code)]
pub fn s2() -> i64 {
    let (seeds, maps) = get_input(true);
    // println!("Seeds {:?}\nmaps {:?}", all_seeds, maps);

    let all_seeds: Vec<Range<i64>> = seeds.iter().step_by(2).enumerate()
        .map(|(i, s)| {
            s.to_owned()..(s+seeds[(i*2)+1]).to_owned()
        }).collect();
    // println!("{:?}", all_seeds);

    let lowest_vals: Vec<i64> = all_seeds.iter().map(|seed_range| {

        let mut lowest_vals_in_range: Vec<i64> = vec![];

        println!("SEED RANGE {:?}", seed_range);
        for seed in seed_range.to_owned() {
            
            let mut next_check = seed.to_owned();
            for map_section in &maps {
                // println!("Seed {} In {}", seed, map_section.0);
                for m in &map_section.1 {
                    if m.range.contains(&next_check) {
                        // println!(
                        //     "For {}, path {} matched {:?} with offset {:?}",
                        //     seed, next_check, m.range, m.offset
                        // );
                        next_check = next_check+m.offset;
                        break;
                    }
                }
                // println!("For {seed}, path {} matched nothing", next_check);
            }
            lowest_vals_in_range.push(next_check);
        }
        lowest_vals_in_range.iter().min().unwrap().to_owned()
    }).collect();
    // println!("Lowest vals {:?}", lowest_vals);

    return lowest_vals.iter().min().unwrap().to_owned();
}

