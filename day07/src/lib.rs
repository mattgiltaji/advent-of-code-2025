use std::fs::File;
use std::io::{BufRead, BufReader};

/// process the lines of tachyon diagrams and return the count of timelines and what happens to the current line
/// Source (S)
/// empty space (.)
/// tachyon beam (|)
/// splitter (^)
///
/// # Examples
///
/// ```
/// let prev = vec!['.', 'S', '.'];
/// let current = vec!['.', '.', '.'];
/// let timelines = vec![0, 0, 0];
/// let (splits, result) = day07::process_tachyons(prev, current, timelines);
/// assert_eq!(splits, vec![0, 1, 0]);
/// assert_eq!(result, vec!['.', '|', '.'])
/// ```
/// ```
/// let prev = vec!['.', '|', '.'];
/// let current = vec!['.', '^', '.'];
/// let timelines = vec![0, 1, 0];
/// let (splits, result) = day07::process_tachyons(prev, current, timelines);
/// assert_eq!(splits, vec![1, 0, 1]);
/// assert_eq!(result, vec!['|', '^', '|'])
/// ```
/// ```
/// let prev = vec!['.', '|', '.'];
/// let current = vec!['.', '.', '.'];
/// let timelines = vec![0, 1, 0];
/// let (splits, result) = day07::process_tachyons(prev, current, timelines);
/// assert_eq!(splits, vec![0, 1, 0]);
/// assert_eq!(result, vec!['.', '|', '.'])
/// ```
/// ```
/// let prev = vec!['|', '^', '|'];
/// let current = vec!['.', '.', '.'];
/// let timelines = vec![2, 0, 2];
/// let (splits, result) = day07::process_tachyons(prev, current, timelines);
/// assert_eq!(splits, vec![2, 0, 2]);
/// assert_eq!(result, vec!['|', '.', '|'])
/// ```
/// ```
/// let prev = vec!['.', '|', '.', '|', '.'];
/// let current = vec!['.', '^', '.', '^', '.'];
/// let timelines = vec![0, 1, 0, 1, 0];
/// let (splits, result) = day07::process_tachyons(prev, current, timelines);
/// assert_eq!(splits, vec![1, 0, 2, 0, 1]);
/// assert_eq!(result, vec!['|', '^', '|', '^', '|'])
/// ```
pub fn process_tachyons(
    prev: Vec<char>,
    current: Vec<char>,
    timelines: Vec<u64>,
) -> (Vec<u64>, Vec<char>) {
    let mut result: Vec<char> = Vec::new();
    let mut new_timelines: Vec<u64> = Vec::new();

    let mut skip: bool = false;
    for (i, c) in prev.iter().enumerate() {
        if skip {
            skip = false;
            continue;
        }
        match c {
            '.' => {
                result.push(current[i]);
                new_timelines.push(timelines[i]);
            }
            '^' => {
                result.push(current[i]);
                new_timelines.push(timelines[i]);
            }
            'S' => {
                result.push('|');
                new_timelines.push(1);
            }
            '|' => {
                let d = current[i];
                if d == '^' {
                    //splitter, gotta change the previous to a beam
                    result.pop().unwrap();
                    result.push('|');
                    result.push('^');
                    result.push('|');

                    //add the timelines
                    let x = new_timelines.pop().unwrap();
                    let y = timelines[i];
                    let z = timelines[i + 1];

                    //left side gets its existing + the newly split timelines
                    new_timelines.push(x + y);
                    //center has timelines blocked because it split
                    new_timelines.push(0);
                    //right side gets its existing + the newly split timelines
                    new_timelines.push(y + z);
                    //don't look at the next prev value, we've already calculated the right side of the split
                    skip = true;
                } else {
                    //empty space, continue the beam
                    result.push('|');
                    new_timelines.push(timelines[i]);
                }
            }
            other => panic!("weird char found - {other}"),
        }
    }
    println!("processed {prev:?} and transformed to {result:?} with timelines {new_timelines:?}");
    (new_timelines, result)
}

pub fn get_total_tachyon_splits(input: File) -> u64 {
    let mut result: u64 = 0;
    let buf = BufReader::new(input);
    let mut data: Vec<Vec<char>> = Vec::new();
    let mut timelines: Vec<u64> = Vec::new();

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
            for _ in 0..prev.len() {
                timelines.push(0);
            }
            continue;
        }
        let (tls, updated) = process_tachyons(prev, current, timelines);
        timelines = tls;
        prev = updated;
    }

    //count up the timelines in the final line
    for x in timelines {
        result += x;
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
        assert_eq!(result, 40);
    }

    #[test]
    fn get_total_rolls_real_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/input.txt");
        let data = File::open(path).expect("input.txt file missing");
        let result = get_total_tachyon_splits(data);
        assert_eq!(result, 12895232295789);
    }
}
