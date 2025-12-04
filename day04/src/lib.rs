use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

/// Returns the count of movable rolls in the current line
/// previous and next lines are needed for determining if a roll in the current line can move
/// a roll can move if the 8 adjacent positions contain fewer than 4 rolls
///
/// # Examples
///
/// ```
/// let rolls = day04::get_roll_count_by_line(String::from(".........."), String::from("..@@.@@@@."), String::from("@@@.@.@.@@"));
/// assert_eq!(rolls, 5)
/// ```
/// ```
/// let rolls = day04::get_roll_count_by_line(String::from("@@.@@@@.@@"), String::from(".@@@@@@@.@"), String::from(".@.@.@.@@@"));
/// assert_eq!(rolls, 0)
/// ```
/// ```
/// let rolls = day04::get_roll_count_by_line(String::from(".@@@@@@@@."), String::from("@.@.@@@.@."), String::from(".........."));
/// assert_eq!(rolls, 3)
/// ```
/// ```
/// let rolls = day04::get_roll_count_by_line(String::from("@.@@@@..@."), String::from("@@.@@@@.@@"), String::from(".@@@@@@@.@"));
/// assert_eq!(rolls, 2)
/// ```
pub fn get_roll_count_by_line(prev: String, curr: String, next: String) -> u64 {
    let mut rolls: u64 = 0;
    let mut my_prev: Vec<char> = prev.chars().collect();
    let mut my_curr: Vec<char> = curr.chars().collect();
    let mut my_next: Vec<char> = next.chars().collect();

    let size = my_curr.len();
    //add leader and trailer dots to avoid overflow
    my_prev.insert(0, '.');
    my_curr.insert(0, '.');
    my_next.insert(0, '.');
    my_prev.push('.');
    my_curr.push('.');
    my_next.push('.');

    for i in 1..size+1 {
        if my_curr[i] != '@' {
            continue;
        }
        let mut other_rolls = 0;
        //println!("evaluation {i}'s adjacent rolls");
        for j in [i - 1, i, i + 1] {
            if my_prev[j] == '@' {
                //println!("prev {j} is a roll");
                other_rolls += 1;
            }
            if my_curr[j] == '@' {
                //println!("current {j} is a roll");
                other_rolls += 1;
            }
            if my_next[j] == '@' {
                //println!("next {j} is a roll");
                other_rolls += 1;
            }
        }
        //println!("roll {i} has {other_rolls} nearby rolls");
        //other_rolls now has all adjacent rolls, including the middle one, so compare to 5, not 4
        if other_rolls < 5 {
            rolls += 1;
        }
    }

    rolls
}

pub fn get_total_rolls(input: File) -> u64 {
    let mut result: u64 = 0;
    let buf = BufReader::new(input);
    let mut prev_line: String;
    let mut curr_line: String = String::new();
    let mut next_line: String = String::new();

    for line in buf.lines() {
        let validated_line = line.expect("weird line");
        prev_line = curr_line;
        curr_line = next_line;
        next_line = validated_line;
        if curr_line.is_empty() {
            //first line ever read, skip it because we need the next line
            //but set it up full of dots for the next pass
            let size = next_line.len();
            let filler = vec!['.'; size];
            curr_line = filler.into_iter().collect();
            continue;
        }

        let line_rolls =
            get_roll_count_by_line(prev_line.clone(), curr_line.clone(), next_line.clone());
        result += line_rolls;
        //println!("got {line_rolls} from processing {curr_line}");
    }
    //need to do one final run here with next_line full of dots
    if !curr_line.is_empty() {
        let size = curr_line.len();
        let filler = vec!['.'; size];
        prev_line = curr_line;
        curr_line = next_line;
        next_line = filler.into_iter().collect();
        let line_rolls =
            get_roll_count_by_line(prev_line.clone(), curr_line.clone(), next_line.clone());
        result += line_rolls;
    }

    result
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn get_total_rolls_example_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test1.txt");
        let data = File::open(path).expect("test1.txt file missing");
        let result = get_total_rolls(data);
        assert_eq!(result, 13);
    }

    #[test]
    fn get_total_rolls_real_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/input.txt");
        let data = File::open(path).expect("input.txt file missing");
        let result = get_total_rolls(data);
        assert_eq!(result, 1467);
    }
}
