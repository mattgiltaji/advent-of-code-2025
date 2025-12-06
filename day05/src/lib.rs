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


pub fn get_total_fresh(input: File) -> u64 {
    let mut result: u64 = 0;
    let buf = BufReader::new(input);
    let mut ranges:Vec<(u64,u64)> = Vec::new();
    let mut ingredients:Vec<u64> = Vec::new();
    let mut range_time = true;

    for line in buf.lines() {
        let validated_line = line.expect("weird line");
        if validated_line.is_empty() {
            range_time = false;
            continue;
        }
        if range_time{
            let (s, e) = validated_line.split_once('-').unwrap();
            let start:u64 = s.parse().unwrap();
            let end:u64 = e.parse().unwrap();
            ranges.push((start, end));
        } else {
            let ingredient:u64 = validated_line.parse().unwrap();
            ingredients.push(ingredient);
        }
    }

    //we might need to optimize ranges here... like sort it, merge the overlaps, etc

    for i in ingredients {
        if check_freshness(i, ranges.clone()){
            result += 1;
        }
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
        assert_eq!(result, 3);
    }

    #[test]
    fn get_total_rolls_real_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/input.txt");
        let data = File::open(path).expect("input.txt file missing");
        let result = get_total_fresh(data);
        assert_eq!(result, 679);
    }
}
