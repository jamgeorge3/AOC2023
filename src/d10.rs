use std::fs;
use std::collections::HashMap;
use std::cmp::min;
use std::cmp::max;
use super::tools::shapes;

// const TEST_INPUT: &str = 
// ".F-7.
// .S.|.
// .|.|.
// .L-J.
// .....";

// const TEST_INPUT: &str = 
// "...........
// .S-------7.
// .|F-----7|.
// .||.....||.
// .||.....||.
// .|L-7.F-J|.
// .|..|.|..|.
// .L--J.L--J.
// ...........";

const TEST_INPUT: &str =
".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";

// const TEST_INPUT: &str = 
// "..F7.
// .FJ|.
// SJ.L7
// |F--J
// LJ...";


fn get_input(test:bool) -> Vec<Vec<char>> {
    let test_input = TEST_INPUT.to_owned();
    let file_input = fs::read_to_string("data/d10.input")
        .unwrap().to_owned();

    let input_to_use = if test { test_input } else { file_input };

    let pipe_map = input_to_use.lines()
        .map(|x| x.chars().collect::<Vec<char>>() )
        .collect::<Vec<Vec<char>>>();
    
    return pipe_map;
}


fn get_next_coord(
    prev_coord: [i32; 2],
    pipe_coord: [i32; 2], 
    pipe_connectors: Option<&[[i32; 2]; 2]>,
) -> Option<[i32; 2]> {
    if pipe_connectors.is_none() {return None};

    let pipe_conns = pipe_connectors.unwrap();

    let connector_to_use = if 
        prev_coord[0] == (pipe_coord[0] + pipe_conns[0][0])
        && prev_coord[1] == (pipe_coord[1] + pipe_conns[0][1])
    {Some(1)} else if 
        prev_coord[0] == (pipe_coord[0] + pipe_conns[1][0])
        && prev_coord[1] == (pipe_coord[1] + pipe_conns[1][1])
    {Some(0)} else {None};

    if connector_to_use.is_none() {return None}
    // println!("USING PIPE CONNECTOR {:?} {:?}", connector_to_use, pipe_connectors[connector_to_use]);

    return Some([
        (pipe_coord[0] + pipe_conns[connector_to_use.unwrap()][0]), 
        (pipe_coord[1] + pipe_conns[connector_to_use.unwrap()][1])
    ])
}


#[allow(dead_code)]
pub fn s1() -> i64 {
    let pipe_types: HashMap<char, [[i32; 2]; 2]> = HashMap::from([
        // (pipe, pipe connectors) where connectors are offsets of [[row,col],[row,col]]
        ('|', [[-1, 0], [1, 0]]),
        ('-', [[0, -1], [0, 1]]),
        ('L', [[-1, 0], [0, 1]]),
        ('J', [[-1, 0], [0, -1]]),
        ('7', [[0, -1], [1, 0]]),
        ('F', [[0, 1], [1, 0]])
    ]);

    let pipe_map: Vec<Vec<char>> = get_input(false);

    // Find the S
    let mut starting_coord: [i32; 2] = [0,0];
    for (row_idx, row) in pipe_map.iter().enumerate() {
        if let Some(row_col) = row.iter().position(|x| *x=='S') { 
            starting_coord=[row_idx as i32, row_col as i32];
            break;
        };
    }

    // Find first connecting pipe
    let mut first_pipe: Option<[i32; 2]> = None;
    
    let mut prev_coord = [0,0];
    let mut curr_coord = [0,0];
    let mut next_coord = [0,0];
    
    for row in 
    max(starting_coord[0]-1, 0)..=min(starting_coord[0]+1, pipe_map.len() as i32) {
        for col in 
        max(starting_coord[1]-1, 0)..=min(starting_coord[1]+1, pipe_map[0].len() as i32) {
            if let Some(next_coord) = get_next_coord(
                starting_coord,
                [row, col],
                pipe_types.get(&pipe_map[row as usize][col as usize])    
            ) {
                first_pipe = Some([row, col]);
                prev_coord = [row, col];
                curr_coord = next_coord;
                break;
            }
        }
        if first_pipe.is_some() {break};
    }

    // Find entire path
    // We just took a step
    let mut steps_taken = 1;

    while next_coord != starting_coord {

        next_coord = get_next_coord(
            prev_coord, 
            curr_coord, 
            pipe_types.get(&pipe_map[curr_coord[0] as usize][curr_coord[1] as usize])
        ).unwrap();

        prev_coord = curr_coord;
        curr_coord = next_coord;

        steps_taken += 1;
    }


    return (steps_taken+1)/2;
}


#[allow(dead_code)]
pub fn s2() -> i64 {
    let pipe_types: HashMap<char, [[i32; 2]; 2]> = HashMap::from([
        // (pipe, pipe connectors) where connectors are offsets of [[row,col],[row,col]]
        ('|', [[-1, 0], [1, 0]]),
        ('-', [[0, -1], [0, 1]]),
        ('L', [[-1, 0], [0, 1]]),
        ('J', [[-1, 0], [0, -1]]),
        ('7', [[0, -1], [1, 0]]),
        ('F', [[0, 1], [1, 0]])
    ]);

    let mut pipe_map: Vec<Vec<char>> = get_input(true);

    // Find the S
    let mut starting_coord: [i32; 2] = [0,0];
    for (row_idx, row) in pipe_map.iter().enumerate() {
        if let Some(row_col) = row.iter().position(|x| *x=='S') { 
            starting_coord=[row_idx as i32, row_col as i32];
            break;
        };
    }

    // Find pipes connecting to S
    let mut prev_coord = [0,0];
    let mut curr_coord = [0,0];
    let mut next_coord = [0,0];

    let mut replace_s_with: Vec<[i32; 2]> = vec![];
    
    for row in 
    max(starting_coord[0]-1, 0)..=min(starting_coord[0]+1, pipe_map.len() as i32) {
        for col in 
        max(starting_coord[1]-1, 0)..=min(starting_coord[1]+1, pipe_map[0].len() as i32) {
            if let Some(next_coord) = get_next_coord(
                starting_coord,
                [row, col],
                pipe_types.get(&pipe_map[row as usize][col as usize])    
            ) {
                // S connects to 2 pipes. We need to replace S with its pipe type, so 
                // need to loop through 2 connections to S 
                replace_s_with.push([
                    row-starting_coord[0],
                    col-starting_coord[1]
                ]);

                prev_coord = [row, col];
                curr_coord = next_coord;
            }
        }
    }

    for (pipe, connector_type) in pipe_types.iter() {
        if 
            (connector_type[0] == replace_s_with[0] || connector_type[1] == replace_s_with[0])
            && (connector_type[0] == replace_s_with[1] || connector_type[1] == replace_s_with[1])
        {
            pipe_map[starting_coord[0] as usize][starting_coord[1] as usize] = *pipe;
        }
    }

    // Find entire path
    let mut coords_in_path: Vec<[i32; 2]> = Vec::from([starting_coord, prev_coord]);
    while next_coord != starting_coord {
        coords_in_path.push(curr_coord);

        next_coord = get_next_coord(
            prev_coord, 
            curr_coord, 
            pipe_types.get(&pipe_map[curr_coord[0] as usize][curr_coord[1] as usize])
        ).unwrap();

        prev_coord = curr_coord;
        curr_coord = next_coord;
    }

    let mut enclosed_count = 0;
    for (row_idx, row) in pipe_map.iter().enumerate() {

        let mut enclosed: bool = false;
        let mut angle_val: i32 = 0;
        for (col_idx, char) in row.iter().enumerate() {

            if coords_in_path.contains(&[row_idx as i32, col_idx as i32]) {
                if *char == '|' {
                    enclosed = !enclosed;
                    angle_val = 0;
                } else {
                    // Get the pipe's row offset (sends north/south)
                    let row_adjust: i32 = pipe_types.get(&pipe_map[row_idx][col_idx]).unwrap().iter()
                        .map(|x| x[0] ).collect::<Vec<i32>>().iter().sum();
                    angle_val += row_adjust;
                    if angle_val.abs() == 0 {
                        enclosed = !enclosed;
                    } else if angle_val % 2 == 0 {
                        angle_val = 0;
                    }
                }
            } else if enclosed {
                enclosed_count += 1;
            };
        }
    }
    
    // let (area, perim) = shapes::find_area_and_perim(&coords_in_path);
    
    // println!(
    //     "AREA {:?} PERIM {:?} POINTS IN PATH {:?} POINTS ENCLOSED {:?}", 
    //     area, perim, coords_in_path.len(), area-(perim/2.0)
    // );
    // return 0;

    return enclosed_count;
}

