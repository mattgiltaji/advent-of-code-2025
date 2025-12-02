use std::{
    io::BufRead,
    io::BufReader,
    fs::File
};

/// Returns true if the id is valid.
/// A valid ID is **not** made up of the same substring repeated twice
///
/// # Examples
///
/// ```
/// let good1 = day02::is_valid_id(123321);
/// assert!(good1);
/// ```
/// ```
/// let good2 = day02::is_valid_id(101);
/// assert!(good2);
/// ```
/// ```
/// let bad = day02::is_valid_id(13121312);
/// assert!(!bad);
/// ```
pub fn is_valid_id(input: u64) -> bool {
    let str_input = input.to_string();
    let len = str_input.chars().count();
    if len % 2 == 1 {
        // odd number can't be a duplicated substring
        true
    } else {
        let (front, back) = str_input.trim().split_at(len / 2);
        front != back
    }
}

/// Returns a vector of invalid IDs from an inclusive range between start and end inputs
/// ID validity determined by day02::is_valid_id()
///
/// # Examples
/// ```
/// let results = day02::get_invalid_ids_from_range(11,22);
/// assert_eq!(results, [11, 22])
/// ```
/// ```
/// let results = day02::get_invalid_ids_from_range(95,115);
/// assert_eq!(results, [99])
/// ```
/// ```
/// let results = day02::get_invalid_ids_from_range(1698522,1698528);
/// assert!(results.is_empty())
/// ```
pub fn get_invalid_ids_from_range(start:u64, end:u64) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();
    for i in start..= end {
        if !is_valid_id(i) {
            result.push(i);
        }
    }
    result
}

/// Parses a string of the form "start-end" and returns a tuple of the start and end
///
/// # Examples
///
/// ```
/// let results = day02::parse_range_from_string(String::from("998-1012"));
/// assert_eq!(results, (998, 1012))
/// ```
/// ```
/// let results = day02::parse_range_from_string(String::from("2121212118-2121212124"));
/// assert_eq!(results, (2121212118, 2121212124))
/// ```
pub fn parse_range_from_string(input: String) -> (u64, u64) {
    println!("parsing range from {input}");
    let parts: Vec<&str> = input.split('-').collect();
    let front = parts[0].trim();
    let back = parts[1].trim();
    let start:u64 = front.parse().expect("unable to parse front of range");
    let end:u64 = back.parse().expect("unable to parse back of range");
    (start, end)
}

pub fn sum_invalid_ids(input:File) -> u64 {
    let mut result:u64 = 0;
    let buf = BufReader::new(input);
    for line in buf.lines() {
        let validated_line = line.expect("weird line");
        let ranges: Vec<&str> = validated_line.split(",").collect();
        for range in ranges {
            let (start, end) = parse_range_from_string(range.to_string());
            let invalid_ids = get_invalid_ids_from_range(start, end);
            for id in invalid_ids {
                result += id;
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn sum_invalid_ids_example_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test1.txt");
        let data = File::open(path).expect("test1.txt file missing");
        let result = sum_invalid_ids(data);
        assert_eq!(result, 1227775554);
    }

    #[test]
    fn sum_invalid_ids_real_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/input.txt");
        let data = File::open(path).expect("input.txt file missing");
        let result = sum_invalid_ids(data);
        assert_eq!(result, 64215794229);
    }
}
