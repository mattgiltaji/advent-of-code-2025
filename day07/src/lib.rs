use std::fs::File;
use std::io::{BufRead, BufReader};

/// process the lines of tachyon diagrams and return the count of splits and what happens to the current line
/// Source (S)
/// empty space (.)
/// tachyon beam (|)
/// splitter (^)
///
/// # Examples
///
/// ```
/// let (splits, result) = day07::process_tachyons(vec!['.', 'S', '.'], vec!['.', '.', '.']);
/// assert_eq!(splits, 0);
/// assert_eq!(result, vec!['.', '|', '.'])
/// ```
/// ```
/// let (splits, result) = day07::process_tachyons(vec!['.', '|', '.'], vec!['.', '^', '.']);
/// assert_eq!(splits, 1);
/// assert_eq!(result, vec!['|', '^', '|'])
/// ```
/// ```
/// let (splits, result) = day07::process_tachyons(vec!['.', '|', '.'], vec!['.', '.', '.']);
/// assert_eq!(splits, 0);
/// assert_eq!(result, vec!['.', '|', '.'])
/// ```
/// ```
/// let (splits, result) = day07::process_tachyons(vec!['|', '^', '|'], vec!['.', '.', '.']);
/// assert_eq!(splits, 0);
/// assert_eq!(result, vec!['|', '.', '|'])
/// ```
/// ```
/// let (splits, result) = day07::process_tachyons(vec!['.', '|', '.', '|', '.'], vec!['.', '^', '.', '^', '.']);
/// assert_eq!(splits, 2);
/// assert_eq!(result, vec!['|', '^', '|', '^', '|'])
/// ```
pub fn process_tachyons(prev: Vec<char>, current: Vec<char>) -> (u64, Vec<char>) {
    let mut result: Vec<char> = Vec::new();
    let mut count: u64 = 0;

    let mut skip: bool = false;
    for (i, c) in prev.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        }
        match c {
            '.' => result.push(current[i]),
            '^' => result.push(current[i]),
            'S' => result.push('|'),
            '|' => {
                let d = current[i];
                if d == '^' {
                    //splitter, gotta change the previous to a beam and increment the splits
                    count += 1;
                    result.pop().unwrap();
                    result.push('|');
                    result.push('^');
                    result.push('|');
                    //don't look at the next prev value, we're already a beam
                    skip = true;
                } else {
                    //empty space, continue the beam
                    result.push('|');
                }
            }
            other => panic!("weird char found - {other}"),
        }
    }
    //println!("processed {prev:?} and found {count} splits and transformed to {result:?}");
    (count, result)
}

pub fn get_total_tachyon_splits(input: File) -> u64 {
    let mut result: u64 = 0;
    let buf = BufReader::new(input);
    let mut data: Vec<Vec<char>> = Vec::new();

    //gather the whole file into matrix of chars
    for line in buf.lines() {
        let validated_line = line.expect("weird line");
        let split_data: Vec<char> = validated_line.chars().collect();
        data.push(split_data);
    }

    let mut prev: Vec<char> = Vec::new();

    for current in data {
        if prev.is_empty() {
            prev = current;
            continue;
        }
        let (splits, updated) = process_tachyons(prev, current);
        result += splits;
        prev = updated;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn get_total_rolls_example_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test1.txt");
        let data = File::open(path).expect("test1.txt file missing");
        let result = get_total_tachyon_splits(data);
        assert_eq!(result, 21);
    }

    #[test]
    fn get_total_rolls_real_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/input.txt");
        let data = File::open(path).expect("input.txt file missing");
        let result = get_total_tachyon_splits(data);
        assert_eq!(result, 1555);
    }
}
