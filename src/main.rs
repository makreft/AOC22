use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn compare_elve_calories(a: &(i32, i32), b: &(i32, i32)) -> std::cmp::Ordering
{
    let test = a.1.cmp(&b.1);
    return test;
}

#[allow(dead_code)]
fn get_answer_to_day_one() -> i32
{
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
    return sum_of_top_three_elves; // --> answer 1.2
    // return all_elves[all_elves.len()-1].0, all_elves[all_elves.len()-1].1; // -->answer 1.1
}

#[derive(Debug, Copy, Clone)]
enum RPS
{
    Rock=1,
    Paper,
    Scissor,
    Default=99
}

fn convert_input_to_score(oponent: &String, me: &String) -> i32
{
    let look_up_table = vec![("A".to_string(), "X".to_string(), RPS::Rock),
                                                         ("B".to_string(), "Y".to_string(), RPS::Paper),
                                                         ("C".to_string(), "Z".to_string(), RPS::Scissor)];

    let mut oponents_move = RPS::Default;
    let mut own_move = RPS::Default;
    for ele in look_up_table
    {
        if &ele.0 == oponent
        {
            oponents_move = ele.2;
        }
        if &ele.1 == me
        {
            own_move = ele.2;
        }
    }
    if own_move as i32 == oponents_move as i32
    {
        // println!("draw");
        return 3 + own_move as i32;
    }
    else if (own_move as i32 - oponents_move as i32) > 0 && ((own_move as i32 - oponents_move as i32).abs() == 1) 
    {
        // println!("own wins");
        return 6 + own_move as i32;
    }
    else if (own_move as i32 - oponents_move as i32 ) < 0 && ((own_move as i32 - oponents_move as i32).abs() == 2)
    {
        // println!("own wins");
        return 6 + own_move as i32;
    }
    else 
    {
        // println!("own looses");
        return own_move as i32;
    }


    // println!("own{}", own_move as i32);
    // println!("diff: {}", oponents_move as i32 - own_move as i32);
    
    // println!("{}, {}", oponent, me );
}

fn get_answer_to_day_two() -> i32
{
    let filename = "input/level2.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut sum_of_score = 0;
    for(_, line) in reader.lines().enumerate()
    {
        let line_str = line.unwrap();
        let test = line_str.split(" ").collect::<Vec<&str>>();
        sum_of_score = sum_of_score + convert_input_to_score(&test[0].to_string(), &test[1].to_string());
    }
    return sum_of_score;
}

fn main() {
    // print!("{}", get_answer_to_day_one());
    println!("{}", get_answer_to_day_two());
}

