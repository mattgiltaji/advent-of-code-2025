use std::{
    fs::File,
    io::BufRead,
    io::BufReader
};

/// Returns maximum joltage from a battery bank
/// joltage is the combination of 2 digits in the passed-in string,
///   with the left digit becoming the tens and right becoming the ones
///
/// # Examples
///
/// ```
/// let joltage = day03::get_max_joltage(String::from("987654321111111"));
/// assert_eq!(joltage, 98);
/// ```
/// ```
/// let joltage = day03::get_max_joltage(String::from("123456789"));
/// assert_eq!(joltage, 89);
/// ```
/// ```
/// let joltage = day03::get_max_joltage(String::from("9239444444448"));
/// assert_eq!(joltage, 99);
/// ```
pub fn get_max_joltage(bank: String) -> u32 {
    let mut first:u32 = 0;
    let mut second:u32 = 0;
    const RADIX:u32 = 10;
    let mut  mut_bank = bank;
    //we won't be able to make a 2 digit number if we use the final digit as first,s o save it for later
    let last = mut_bank.pop().expect("empty string");

    for b in mut_bank.chars() {
        let current = b.to_digit(RADIX).expect("non-digit char found");
        if current > first {
            first = current;
        }
    }
    //add the final digit back
    mut_bank.push(last);
    let f = char::from_digit(first, RADIX).unwrap();

    //only use digits after our selected first one
    let (_, remaining) = mut_bank.split_once(f).unwrap();

    for r in remaining.chars() {
        let current = r.to_digit(RADIX).expect("non-digit char found");
        if current > second {
            second = current;
        }
    }
    let result = first * 10 + second;
    result
}

pub fn get_total_joltage(input:File) -> u32 {
    let mut result:u32 = 0;
    let buf = BufReader::new(input);
    for line in buf.lines() {
        let validated_line = line.expect("weird line");
        let line_jolt = get_max_joltage(validated_line);
        result += line_jolt;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::path::Path;

    #[test]
    fn get_total_joltage_example_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test1.txt");
        let data = File::open(path).expect("test1.txt file missing");
        let result = get_total_joltage(data);
        assert_eq!(result, 357);
    }

    #[test]
    fn get_total_joltage_real_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/input.txt");
        let data = File::open(path).expect("input.txt file missing");
        let result = get_total_joltage(data);
        assert_eq!(result, 17321);
    }
}
