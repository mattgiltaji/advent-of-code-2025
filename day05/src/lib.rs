use std::fs::File;
use std::io::{BufRead, BufReader};

/// Returns true if ingredient falls into any of the inclusive ranges
///
/// # Examples
///
/// ```
/// let fresh = day05::check_freshness(1, vec![(3,5)]);
/// assert!(!fresh);
/// ```
/// ```
/// let fresh = day05::check_freshness(5, vec![(3,5)]);
/// assert!(fresh);
/// ```
/// ```
/// let fresh = day05::check_freshness(8, vec![(3,5), (10,14), (16,20)]);
/// assert!(!fresh);
/// ```
pub fn check_freshness(ingredient:u64, ranges:Vec<(u64, u64)>) -> bool {
    let mut is_fresh = false;

    for (lower, upper) in ranges {
        if ingredient < lower {
            continue;
        }
        if ingredient <= upper {
            is_fresh = true;
            break;
        }
    }

    is_fresh
}

/// Merges overlapping intervals and returns the shortened interval vector
///
/// # Examples
/// ```
/// let mut input: Vec<(u64, u64)> = vec![(3,5), (10,14), (16,20), (12,18)];
/// let result = day05::merge_overlapping_intervals(&mut input);
/// assert_eq!(result[0], (3,5));
/// assert_eq!(result[1], (10,20));
/// ```
pub fn merge_overlapping_intervals(arr: &mut Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut result: Vec<(u64, u64)> = Vec::new();
    arr.sort_by(|(a, _),(c,_)| a.cmp(&c));
    result.push(arr[0].clone());
    let size = arr.len();

    for i in 1..size {
        let (a, b) = arr[i].clone();
        let last:usize = result.len() - 1;
        let (x, y) = result[last];
        if a >= x && a <= y {
            //a is in the current interval
            if b > y {
                //b is outside the current interval, expand it
                result[last] = (x, b);
            } //b is inside the current interval, fully overlapped, move on to the next
        } else {
            result.push((a,b));
        }
    }
    result
}

pub fn get_total_fresh(input: File) -> u64 {
    let mut result: u64 = 0;
    let buf = BufReader::new(input);
    let mut ranges:Vec<(u64,u64)> = Vec::new();

    for line in buf.lines() {
        let validated_line = line.expect("weird line");
        if validated_line.is_empty() {
            //ignore ingredient ids, we'll populate them ourselves
            break;
        }

        let (s, e) = validated_line.split_once('-').unwrap();
        let start:u64 = s.parse().unwrap();
        let end:u64 = e.parse().unwrap();
        ranges.push((start, end));

    }
    //consolidate ranges
    ranges = merge_overlapping_intervals(&mut ranges);

    for (i, j) in ranges.clone() {
        result += j - i + 1;
    }

    result
}
#[cfg(test)]
mod tests {
    use std::path::Path;
    use super::*;

    #[test]
    fn get_total_fresh_example_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test1.txt");
        let data = File::open(path).expect("test1.txt file missing");
        let result = get_total_fresh(data);
        assert_eq!(result, 14);
    }

    #[test]
    fn get_total_rolls_real_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/input.txt");
        let data = File::open(path).expect("input.txt file missing");
        let result = get_total_fresh(data);
        assert_eq!(result, 358155203664116);
    }
}
