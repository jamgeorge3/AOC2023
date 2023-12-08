use std::fs;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn s1() -> u64 {

    let bounds: HashMap<&str, i32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);

    let mut grand_total: u64 = 0;

    for ln_raw in fs::read_to_string("data/d2.input").unwrap().lines() {
        let mut game_valid: bool = true;
        let line: Vec<&str> = ln_raw.split(": ").collect::<Vec<_>>();
        let idx: u64 = line[0].split(" ").collect::<Vec<_>>()[1].parse::<u64>().unwrap();
        let game_params_chunks: Vec<&str> = line[1].split("; ").collect::<Vec<_>>();
        let mut game_params: Vec<&str> = vec![];
        for g in game_params_chunks {
            game_params.extend(g.split(", ").collect::<Vec<_>>());
        }
        for g in game_params {
            let balls: Vec<&str> = g.split(" ").collect::<Vec<_>>();
            let ball_count: i32 = balls[0].parse::<i32>().unwrap();
            let bound: &i32 = bounds.get(balls[1]).unwrap();
            if ball_count > *bound {
                game_valid = false;
                break
            }
        }

        if game_valid {
            grand_total += idx;
        }
    }

    return grand_total;
}


#[allow(dead_code)]
pub fn s2() -> u64 {
    let mut grand_total: u64 = 0;

    for ln_raw in fs::read_to_string("data/d2.input").unwrap().lines() {

        let mut mins: HashMap<&str, i32> = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]);

        let line: Vec<&str> = ln_raw.split(": ").collect::<Vec<_>>();

        let game_params_chunks = line[1].split("; ").collect::<Vec<_>>();
        let mut game_params: Vec<&str> = vec![];
        for g in game_params_chunks {
            game_params.extend(g.split(", ").collect::<Vec<_>>());
        }

        for g in game_params {
            let balls: Vec<&str> = g.split(" ").collect::<Vec<_>>();
            let ball_count: i32 = balls[0].parse::<i32>().unwrap();

            let curr_min: &i32 = mins.get(balls[1]).unwrap();
            if curr_min < &ball_count {
                *mins.get_mut(balls[1]).unwrap() = ball_count;
            }
        }

        let mut subtotal: i32 = 1;
        for v in mins.values() {
            subtotal *= v;
        }

        grand_total += u64::try_from(subtotal).unwrap();
    }

    return grand_total;
}

