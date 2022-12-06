use std::fs::File;
use std::io::{BufRead, BufReader};

fn compare_elve_calories(a: &(i32, i32), b: &(i32, i32)) -> std::cmp::Ordering
{
    let test = a.1.cmp(&b.1);
    return test;
}

fn main() {
    let filename = "input/level1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut calories_of_curr_elve: i32 = 0;
    let mut number_of_elve: i32 = 1;
    let mut all_elves: Vec<(i32 /*number of elve*/ , i32 /*calories of elve*/)> = Vec::new();

    for(_, line) in reader.lines().enumerate()
    {
        let _line_content = line.unwrap();
        // println!("{}", _line_content);
        if _line_content.len() == 0
        {
            all_elves.push((number_of_elve, calories_of_curr_elve));
            calories_of_curr_elve = 0;
            number_of_elve = number_of_elve + 1;
        }
        else
        {
            calories_of_curr_elve = calories_of_curr_elve + _line_content.parse::<i32>().unwrap();
        }
    }

    all_elves.sort_by(compare_elve_calories);
    let sum_of_top_three_elves = all_elves[all_elves.len()-1].1 + all_elves[all_elves.len()-2].1 + all_elves[all_elves.len()-3].1;
    // print!("{}", sum_of_top_three_elves); // --> answer 1.2
    // println!("{} {}", all_elves[all_elves.len()-1].0, all_elves[all_elves.len()-1].1); // -->answer 1.1
}
