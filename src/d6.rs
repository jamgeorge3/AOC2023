use std::fs;
use regex::Regex;


const TEST_INPUT: &str = 
"Time:      7  15   30
Distance:  9  40  200";


fn get_input(test:bool) -> (Vec<i64>,  Vec<i64>) {
    let test_input = TEST_INPUT.to_owned();

    let file_input = fs::read_to_string("data/d6.input")
        .unwrap().to_owned();

    let num_re: Regex = Regex::new(r"\d+").unwrap();

    let input_to_use = if test { test_input } else { file_input }; 
    let lines: Vec<&str> = input_to_use.lines().collect();
    
    let rows: Vec<Vec<i64>> = lines.iter().map(|x| {
        num_re.find_iter(x)
            .map(|x| x.as_str().parse::<i64>().unwrap())
            .collect()
    }).collect();

    let match_durations = rows[0].to_owned();
    let records = rows[1].to_owned();

    return (match_durations, records);
}


#[allow(dead_code)]
pub fn s1() -> i64 {
    let (match_durations, records) = get_input(false);

    let mut grand_total = 1;
    for (i, m_duration) in match_durations.iter().enumerate() {
        let range_to_beat = records[i];

        let mut possible_scores = vec![];
        for t in 0..m_duration.to_owned() {
            possible_scores.push(t*(m_duration-t))
        }

        possible_scores.retain(|&x| x > range_to_beat);

        grand_total *= possible_scores.len() as i64;
    }

    return grand_total;
}


#[allow(dead_code)]
pub fn s2() -> i64 {
    let (match_durations, records) = get_input(false);

    let match_dur_total = match_durations.iter()
        .map(|x| x.to_string()).collect::<String>()
        .parse::<i64>().unwrap();

    let record_total = records.iter()
        .map(|x| x.to_string()).collect::<String>()
        .parse::<i64>().unwrap();
        
    let mut possible_scores = vec![];
    
    /* distance = press_time * (total_match_duration - press_time)
    
    Though, apparently this is a quadratic equation and can be solved
    Much more quickly by calculating press_time for the given distance 
    and time, then just finding all press_times that are between

    But I'm not smart enough for that... so here you go
    */
    for t in 0..match_dur_total.to_owned() {
        possible_scores.push(t*(match_dur_total-t))
    }

    possible_scores.retain(|&x| x > record_total);

    return possible_scores.len() as i64;
}
