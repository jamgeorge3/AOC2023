use std::fs;

const TEST_INPUT: &str = 
"";


fn get_input(test:bool) -> () {
    // let star_re: Regex = Regex::new(r"#").unwrap();

    let test_input = TEST_INPUT.to_owned();
    let file_input = fs::read_to_string("data/d12.input")
        .unwrap().to_owned();

    let input_to_use = if test { test_input } else { file_input };
}


#[allow(dead_code)]
pub fn s1() -> i64 {
    return 0;
}


#[allow(dead_code)]
pub fn s2() -> i64 {
    return 0;
}

