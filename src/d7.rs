use std::fs;
use std::collections::HashMap;
use std::iter::zip;
use std::cmp::max;


const TEST_INPUT: &str = 
"32T3K 765
T55J5 684
KK677 28
KTTJT 220
QQJJA 483";

fn sort_to_rank(hand: Vec<String>, ranks: &mut HashMap<u32, Vec<Vec<String>>>) {
    let cards = hand[0].to_owned();
    let mut card_counts: HashMap<char, i32> = HashMap::new();

    for c in cards.chars() {
        card_counts.entry(c)
            .and_modify(|c| *c+=1).or_insert(1);
    }

    let max_count: &i32 = card_counts.values().max().unwrap();

    let mut i = 0;
    // Five of a kind
    match card_counts.keys().len() {
        // 5 of a kind
        1 => i = 1,
        // 4 of a kind or full house
        2 => if max_count == &4 {i = 2} else {i = 3},
        // 3 of a kind or two pair
        3 => if max_count == &3 {i = 4} else {i = 5},
        // One pair
        4 => i = 6,
        // high card
        5 => i = 7, 
        _ => {}
    }

    ranks.entry(i).or_insert(Vec::new()).push(hand);
}


fn get_input(test:bool) -> Vec<Vec<String>> {
    let test_input = TEST_INPUT.to_owned();

    let file_input = fs::read_to_string("data/d7.input")
        .unwrap().to_owned();

    let input_to_use = if test { test_input } else { file_input }; 
    let lines: Vec<Vec<String>> = input_to_use.lines().map(String::from)
        .map(|x| x.split(" ").map(String::from).collect::<Vec<_>>()).collect::<Vec<_>>();

    return lines;
}


#[allow(dead_code)]
pub fn s1() -> u64 {
    let card_vals = ['A','K','Q','J','T','9','8','7','6','5','4','3','2'];
    let mut i = card_vals.len()+1;
    let card_hierarchy: HashMap<char, i32> = 
        card_vals.iter().map(|&x| { i -= 1; (x, i as i32) } )
        .collect::<HashMap<_, _>>();

    let hands = get_input(true);

    let mut ranks = (hands.len()) as u64;
    let mut total: u64 = 0;

    let mut hand_ranks = HashMap::new();

    for hand in hands {
        sort_to_rank(hand.clone(), &mut hand_ranks);    
    }

    let mut hand_ranks_sorted: Vec<(u32, Vec<Vec<String>>)> = hand_ranks.into_iter().collect();
    // Sort by keys
    hand_ranks_sorted.sort_by(|x,y| 
        x.0.cmp(&y.0));

    for (_, hand_group) in hand_ranks_sorted {

        let hands: Vec<i64> = hand_group.iter()
            .map(
                |x| {
                    let mut i = x[0].len() as u32;
                    x[0].to_owned().chars()
                        .map(|y| card_hierarchy.get(&y).unwrap().clone())
                        .map(|z| { i -= 1; (z as i64)*(card_vals.len() as i64).pow(i)})
                    .collect::<Vec<i64>>()})
            .collect::<Vec<Vec<i64>>>().iter()
            .map(|x| x.to_owned().iter().sum()).collect();
        
        let scores: Vec<i64> = hand_group.iter()
            .map(|x| x[1].parse::<i64>().unwrap()).collect();

        let mut groups: Vec<(i64, i64)> = zip(hands, scores).collect();
            
        groups.sort_by(|x,y| y.0.cmp(&x.0));

        for group in groups {
            total += ranks*(group.1 as u64);
            ranks -= 1;
        }
    }

    return total;
}


fn sort_to_rank_2(hand: Vec<String>, ranks: &mut HashMap<u32, Vec<Vec<String>>>) {
    let cards = hand[0].to_owned();
    let mut card_counts: HashMap<char, i32> = HashMap::new();

    let mut jokers = 0;

    for c in cards.chars() {
        if c == 'J' {
            jokers += 1;
        } else {
            card_counts.entry(c)
                .and_modify(|c| *c+=1).or_insert(1);
        }
        
    }

    let max_count: i32 = card_counts.values().max().unwrap_or(&0).to_owned()+jokers;
    

    let mut i = 0;
    // Five of a kind
    match max(card_counts.keys().len(), 1) {
        // 5 of a kind
        1 => i = 1,
        // 4 of a kind or full house
        2 => if max_count == 4 {i = 2} else {i = 3},
        // 3 of a kind or two pair
        3 => if max_count == 3 {i = 4} else {i = 5},
        // One pair
        4 => i = 6,
        // high card
        5 => i = 7, 
        _ => {}
    }

    ranks.entry(i).or_insert(Vec::new()).push(hand);
}


#[allow(dead_code)]
pub fn s2() -> u64 {
    let card_vals = ['A','K','Q','J','T','9','8','7','6','5','4','3','2','J'];
    let mut i = card_vals.len()+1;
    let card_hierarchy: HashMap<char, i32> = 
        card_vals.iter().map(|&x| { i -= 1; (x, i as i32) } )
        .collect::<HashMap<_, _>>();

    let hands = get_input(false);

    let mut ranks = (hands.len()) as u64;
    let mut total: u64 = 0;

    let mut hand_ranks = HashMap::new();

    for hand in hands {
        sort_to_rank_2(hand.clone(), &mut hand_ranks);    
    }

    let mut hand_ranks_sorted: Vec<(u32, Vec<Vec<String>>)> = hand_ranks.into_iter().collect();
    // Sort by keys
    hand_ranks_sorted.sort_by(|x,y| 
        x.0.cmp(&y.0));

    for (_, hand_group) in hand_ranks_sorted {

        let hands: Vec<i64> = hand_group.iter()
            .map(
                |x| {
                    let mut i = x[0].len() as u32;
                    x[0].to_owned().chars()
                        .map(|y| card_hierarchy.get(&y).unwrap().clone())
                        .map(|z| { i -= 1; (z as i64)*(card_vals.len() as i64).pow(i)})
                    .collect::<Vec<i64>>()})
            .collect::<Vec<Vec<i64>>>().iter()
            .map(|x| x.to_owned().iter().sum()).collect();
        
        let scores: Vec<i64> = hand_group.iter()
            .map(|x| x[1].parse::<i64>().unwrap()).collect();

        let mut groups: Vec<(i64, i64)> = zip(hands, scores).collect();

            
        groups.sort_by(|x,y| y.0.cmp(&x.0));

        for group in groups {
            total += ranks*(group.1 as u64);
            ranks -= 1;
        }
    }

    return total;
}
