use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(dead_code)]
fn compare_elve_calories(a: &(i32, i32), b: &(i32, i32)) -> std::cmp::Ordering {
    let test = a.1.cmp(&b.1);
    return test;
}

#[allow(dead_code)]
fn get_answer_to_day_one() -> i32 {
    let filename = "input/level1.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut calories_of_curr_elve: i32 = 0;
    let mut number_of_elve: i32 = 1;
    let mut all_elves: Vec<(i32 /*number of elve*/, i32 /*calories of elve*/)> = Vec::new();

    for (_, line) in reader.lines().enumerate() {
        let _line_content = line.unwrap();
        // println!("{}", _line_content);
        if _line_content.len() == 0 {
            all_elves.push((number_of_elve, calories_of_curr_elve));
            calories_of_curr_elve = 0;
            number_of_elve = number_of_elve + 1;
        } else {
            calories_of_curr_elve = calories_of_curr_elve + _line_content.parse::<i32>().unwrap();
        }
    }

    all_elves.sort_by(compare_elve_calories);
    let sum_of_top_three_elves = all_elves[all_elves.len() - 1].1
        + all_elves[all_elves.len() - 2].1
        + all_elves[all_elves.len() - 3].1;
    return sum_of_top_three_elves; // --> answer 1.2
                                   // return all_elves[all_elves.len()-1].0, all_elves[all_elves.len()-1].1; // -->answer 1.1
}

#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
enum RPS {
    Rock = 1,
    Paper,
    Scissor,
    Default = 99,
}

#[allow(dead_code)]
impl RPS {
    fn from_i32(value: i32) -> RPS {
        match value {
            1 => RPS::Rock,
            2 => RPS::Paper,
            3 => RPS::Scissor,
            _ => panic!("Unknown value: {}", value),
        }
    }
}

#[allow(dead_code)]
fn convert_input_to_score(oponent: &String, me: &String) -> i32 {
    let look_up_table = vec![
        ("A".to_string(), "X".to_string(), RPS::Rock),
        ("B".to_string(), "Y".to_string(), RPS::Paper),
        ("C".to_string(), "Z".to_string(), RPS::Scissor),
    ];

    let mut own_move = RPS::Default;
    let mut oponents_move = RPS::Default;
    for ele in look_up_table {
        if &ele.0 == oponent {
            oponents_move = ele.2;
        }
        if &ele.1 == me {
            own_move = ele.2;
        }
    }

    if own_move as i32 == RPS::Rock as i32 {
        // Lose
        let new_move = oponents_move as i32 - 1;
        if new_move <= 0 {
            own_move = RPS::from_i32(3);
        } else {
            own_move = RPS::from_i32(new_move);
        }
    } else if own_move as i32 == RPS::Paper as i32 {
        // Draw
        own_move = oponents_move;
    } else if own_move as i32 == RPS::Scissor as i32 {
        // Win
        let new_move = oponents_move as i32 + 1;
        if new_move > 3 {
            own_move = RPS::from_i32(1);
        } else {
            own_move = RPS::from_i32(new_move);
        }
    }

    if own_move as i32 == oponents_move as i32 {
        // println!("draw");
        return 3 + own_move as i32;
    } else if (own_move as i32 - oponents_move as i32) > 0
        && ((own_move as i32 - oponents_move as i32).abs() == 1)
    {
        // println!("own wins");
        return 6 + own_move as i32;
    } else if (own_move as i32 - oponents_move as i32) < 0
        && ((own_move as i32 - oponents_move as i32).abs() == 2)
    {
        // println!("own wins");
        return 6 + own_move as i32;
    } else {
        // println!("own looses");
        return own_move as i32;
    }
}

#[allow(dead_code)]
fn get_answer_to_day_two() -> i32 {
    let filename = "input/level2.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut sum_of_score = 0;
    for (_, line) in reader.lines().enumerate() {
        let line_str = line.unwrap();
        let test = line_str.split(" ").collect::<Vec<&str>>();
        sum_of_score =
            sum_of_score + convert_input_to_score(&test[0].to_string(), &test[1].to_string());
    }
    return sum_of_score;
}

#[allow(dead_code)]
fn get_count_of_compartments(compartment_a: &String, compartment_b: &String) -> i32 {
    let a: Vec<_> = compartment_a.chars().collect();
    let mut ret_val: i32 = 0;
    for ele in a {
        if compartment_b.contains(ele) {
            if ele.is_uppercase() {
                ret_val = ele as i32 - 38;
            } else {
                ret_val = ele as i32 - 96;
            }
            break;
        }
    }
    return ret_val;
}

#[allow(dead_code)]
fn get_count_of_grouped_backpacks(
    backpack_a: &String,
    backpack_b: &String,
    backpack_c: &String,
) -> i32 {
    let a: Vec<_> = backpack_a.chars().collect();
    let b: Vec<_> = backpack_b.chars().collect();

    let mut ret_val: i32 = 0;

    for ele_a in &a {
        for ele_b in &b {
            if backpack_c.contains(*ele_a) && backpack_c.contains(*ele_b) && *ele_a == *ele_b {
                if ele_a.is_uppercase() {
                    ret_val = *ele_a as i32 - 38;
                } else {
                    ret_val = *ele_b as i32 - 96;
                }
            }
        }
    }
    return ret_val;
}

#[allow(dead_code)]
fn get_answer_to_day_three() -> i32 {
    let filename = "input/level3.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum_of_score = 0;
    let mut vec_of_three_backpacks: Vec<String> = Vec::with_capacity(3);

    for (_, line) in reader.lines().enumerate() {
        let line_str = line.unwrap();
        vec_of_three_backpacks.push(line_str);
        if vec_of_three_backpacks.len() == vec_of_three_backpacks.capacity() {
            sum_of_score = sum_of_score
                + get_count_of_grouped_backpacks(
                    &vec_of_three_backpacks[0],
                    &vec_of_three_backpacks[1],
                    &vec_of_three_backpacks[2],
                );
            vec_of_three_backpacks.clear();
        }
    }
    return sum_of_score;
}

#[allow(dead_code)]
fn is_fully_contained(left: &String, right: &String) -> bool {
    let left_splitted = left.split("-").collect::<Vec<&str>>();
    let right_splitted = right.split("-").collect::<Vec<&str>>();
    let l_start = left_splitted[0].parse::<i32>().unwrap();
    let l_stop = left_splitted[1].parse::<i32>().unwrap();
    let r_start = right_splitted[0].parse::<i32>().unwrap();
    let r_stop = right_splitted[1].parse::<i32>().unwrap();
    let mut ret_val = false;

    if l_start <= r_start && l_stop >= r_stop {
        ret_val = true;
    } else if r_start <= l_start && r_stop >= l_stop {
        ret_val = true;
    }
    // println!("Left: {}, {}", l_start, l_stop);
    // println!("Right: {}, {}", r_start, r_stop);
    return ret_val;
}
#[allow(dead_code)]
fn has_overlapping_elements(left: &String, right: &String) -> bool {
    let left_splitted = left.split("-").collect::<Vec<&str>>();
    let right_splitted = right.split("-").collect::<Vec<&str>>();
    let l_start = left_splitted[0].parse::<i32>().unwrap();
    let l_stop = left_splitted[1].parse::<i32>().unwrap();
    let r_start = right_splitted[0].parse::<i32>().unwrap();
    let r_stop = right_splitted[1].parse::<i32>().unwrap();
    let mut ret_val = false;
    let a = l_start..l_stop + 1;
    let b = r_start..r_stop + 1;
    for ele in a {
        if b.contains(&ele) {
            ret_val = true;
            break;
        }
    }
    return ret_val;
}

#[allow(dead_code)]
fn get_answer_to_day_four() -> i32 {
    let filename = "input/level4.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut counter = 0;
    for (_, line) in reader.lines().enumerate() {
        let line_str = line.unwrap();
        let splitted_line = line_str.split(",").collect::<Vec<&str>>();
        let left = splitted_line[0];
        let right = splitted_line[1];
        if has_overlapping_elements(&left.to_string(), &right.to_string()) {
            counter = counter + 1;
        }
    }

    return counter;
}

fn main() {
    // print!("{}", get_answer_to_day_one());
    // println!("{}", get_answer_to_day_two());
    // println!("{}", get_answer_to_day_three());
    println!("{}", get_answer_to_day_four());
}
