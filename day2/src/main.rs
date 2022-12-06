use std::collections::HashMap;

fn main() {

    let loss = 0;
    let win = 6;
    let draw = 3;
    let part1_plays = HashMap::from([
        ("X", 1),
        ("Y", 2),
        ("Z", 3),
    ]);
    let part1_outcomes = HashMap::from([
        ("A X", draw),
        ("A Y", win),
        ("A Z", loss),
    
        ("B X", loss),
        ("B Y", draw),
        ("B Z", win),
    
        ("C X", win),
        ("C Y", loss),
        ("C Z", draw),
    ]);

    let input_data = helpers::get_input_data("resources/day2.txt");
    let split_by_round: Vec<&str> = input_data.split("\n").collect();

    let mut part1_total_score = 0;
    for round in split_by_round.iter() {

        let clean_round = round.trim();
        let outcome_score = part1_outcomes.get(clean_round)
            .expect("failed to identify outcome!");
        let play_score = part1_plays.get(&clean_round[2..3])
            .expect("failed to identify play!");
        part1_total_score += outcome_score + play_score;

    }

    println!("The final score for the strategy book in part 1 is {:?}", part1_total_score);

    let rock = 1;
    let paper = 2;
    let scissor = 3;
    let part2_outcomes = HashMap::from([
        ("X", 0),
        ("Y", 3),
        ("Z", 6),
    ]);
    let part2_plays = HashMap::from([
        ("A X", scissor),
        ("A Y", rock),
        ("A Z", paper),
    
        ("B X", rock),
        ("B Y", paper),
        ("B Z", scissor),
    
        ("C X", paper),
        ("C Y", scissor),
        ("C Z", rock),
    ]);

    let mut part2_total_score = 0;
    for round in split_by_round.iter() {

        let clean_round = round.trim();
        let play_score = part2_plays.get(clean_round)
            .expect("failed to identify play!");
        let outcome_score = part2_outcomes.get(&clean_round[2..3])
            .expect("failed to identify outcome!");
        part2_total_score += outcome_score + play_score;

    }

    println!("The final score for the strategy book in part 2 is {:?}", part2_total_score);

}
