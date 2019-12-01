use std::collections::HashMap;
use std::io;

// #![allow(unused_variables)]
fn main() {

    let mut team_score = HashMap::new();

    team_score.insert(String::from("Pakistan"), 250);
    team_score.insert(String::from("India"), 309);
    team_score.insert(String::from("Newzeland"), 230);

    println!("Enter the team name");
    let mut team  = String::new();
    io::stdin().read_line(&mut team).expect("invalid input");
    let team = team.trim();
    
    println!("enter its run ");
    let mut runs = String::new();
    io::stdin().read_line(&mut runs).expect("invalid input");
    let runs = runs.trim().parse::<i32>().unwrap();

    team_score.entry(String::from(team)).or_insert(runs);

    println!("{:?}",team_score);

    for (key, value) in &team_score {
        println!("{}: {}", key, value);
    }

}