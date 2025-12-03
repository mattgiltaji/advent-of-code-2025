use std::{
    fs::File,
    io::BufRead,
    io::BufReader
};

/// Returns maximum joltage from a battery bank
/// joltage is the combination of 12 digits in the passed-in string,
///
///
/// # Examples
///
/// ```
/// let joltage = day03::get_max_joltage(String::from("98765432111111111111"));
/// assert_eq!(joltage, 987654321111);
/// ```
/// ```
/// let joltage = day03::get_max_joltage(String::from("811111111111119"));
/// assert_eq!(joltage, 811111111119);
/// ```
/// ```
/// let joltage = day03::get_max_joltage(String::from("9239444444448"));
/// assert_eq!(joltage, 939444444448);
/// ```
pub fn get_max_joltage(bank: String) -> u64 {
    println!("getting max from {bank}");
    let mut highest:u32 = 0;
    let mut jolt:u64 = 0;
    const RADIX:u32 = 10;
    let mut stack: Vec<char> = Vec::new();
    let mut  mut_bank = bank;

    //pop off final 12 digits of string and put them in a stack
    for _ in 1..12 {
        let last = mut_bank.pop().expect("empty string");
        stack.push(last);
    }

    //split the remaining string at jolt spot
    //pop top of the stack and append it to remaining string
    //do it again until stack is empty
    loop {
        println!("getting highest digit from substring {mut_bank}");
        //find largest digit in remaining string
        for b in mut_bank.chars() {
            let current = b.to_digit(RADIX).expect("non-digit char found");
            if current > highest {
                highest = current;
            }
        }
        jolt = jolt + highest as u64;
        if stack.is_empty(){
            break;
        }

        //shift jolt to next digit spot
        jolt *= 10;


        //add a stack digit to the end
        let last = stack.pop().unwrap();
        mut_bank.push(last);

        //remove the front of the string, we used that already
        let f = char::from_digit(highest, RADIX).unwrap();
        let (_, remaining) = mut_bank.split_once(f).unwrap();


        mut_bank = remaining.to_string();
        println!("found {highest}, jolt: {jolt}, remaining search:{mut_bank}");
        highest = 0;
    }
    jolt
}

pub fn get_total_joltage(input:File) -> u64 {
    let mut result:u64 = 0;
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
        assert_eq!(result, 3121910778619);
    }

    #[test]
    fn get_total_joltage_real_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/input.txt");
        let data = File::open(path).expect("input.txt file missing");
        let result = get_total_joltage(data);
        assert_eq!(result, 171989894144198);
    }
}
