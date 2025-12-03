use std::{
    io::BufRead,
    io::BufReader,
    fs::File,
    sync::LazyLock
};
use fancy_regex::Regex;

/// Returns true if the id is valid.
/// A valid ID is **not** made up of the same substring repeated two or more times
///
/// # Examples
///
/// ```
/// let good = day02::is_valid_id(123321);
/// assert!(good);
/// ```
/// ```
/// let good = day02::is_valid_id(101);
/// assert!(good);
/// ```
/// ```
/// let twice = day02::is_valid_id(13121312);
/// assert!(!twice);
/// ```
/// ```
/// let thrice = day02::is_valid_id(999);
/// assert!(!thrice);
/// ```
/// ```
/// let so_many = day02::is_valid_id(1111111);
/// assert!(!so_many);
/// ```
pub fn is_valid_id(input: u64) -> bool {
    let str_input = input.to_string();
    let trimmed = str_input.trim();
    static RE: LazyLock<Regex> = LazyLock::new (|| Regex::new(r"^(\d+)(\1)+$").expect("weird regex pattern"));
    // regex looks for a sequence of up to 6 digits that are repeated multiple times
    // the number 6 is arbitrary and assumes that we are getting at most a 12-digit number,
    //   so we only need to match up to half the digits.
    let dupes = RE.is_match(trimmed).expect("weird regex match");
    let valid = !dupes;
    valid
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
/// assert_eq!(results, [99, 111])
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
    //println!("found {result:?} invalid IDs");
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
    //println!("parsing range from {input}");
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
        assert_eq!(result, 4174379265);
    }

    #[test]
    fn sum_invalid_ids_real_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/input.txt");
        let data = File::open(path).expect("input.txt file missing");
        let result = sum_invalid_ids(data);
        assert_eq!(result, 85513235135);
    }
}
