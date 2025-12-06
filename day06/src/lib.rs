use std::fs::File;
use std::io::{BufRead, BufReader};

pub enum OctopusOperator {
    Add,
    Multiply,
}

/// do an octopus math problem and return the result
/// octopus math involves applying the same operator to a bunch of arguments and returning the aggregated result
///
/// # Examples
/// ```
/// let result = day06::do_octopus_math(&day06::OctopusOperator::Multiply, vec![123, 45, 6]);
/// assert_eq!(result, 33210);
/// ```
/// ```
/// let result = day06::do_octopus_math(&day06::OctopusOperator::Add, vec![328, 64, 98]);
/// assert_eq!(result, 490);
/// ```
pub fn do_octopus_math(op: &OctopusOperator, args: Vec<u64>) -> u64 {
    let mut result: u64;
    match op {
        OctopusOperator::Add => {
            result = 0;
            for arg in args {
                result += arg;
            }
        }
        OctopusOperator::Multiply => {
            result = 1;
            for arg in args {
                result *= arg;
            }
        }
    }
    result
}

pub fn get_total_octopus_result(input: File) -> u64 {
    let mut result: u64 = 0;
    let buf = BufReader::new(input);
    let mut data: Vec<Vec<u64>> = Vec::new();
    let mut ops: Vec<OctopusOperator> = Vec::new();
    let mut initialized = false;

    for line in buf.lines() {
        let validated_line = line.expect("weird line");
        let split_data: Vec<&str> = validated_line.split_ascii_whitespace().collect();

        //now we have the number of problems, we can properly initialize the data matrix
        if !initialized {
            initialized = true;
            let size = split_data.len();
            (0..size).for_each(|_| {
                let arr: Vec<u64> = Vec::new();
                data.push(arr);
            })
        }

        //if we are the operator line, populate the ops vector
        if split_data[0] == "+" || split_data[0] == "*" {
            for s in split_data {
                match s {
                    "+" => ops.push(OctopusOperator::Add),
                    "*" => ops.push(OctopusOperator::Multiply),
                    other => panic!("weird operator found - {other}"),
                }
            }
            //operator line is last
            break;
        }
        //transpose so each problem is in its own vector
        for (i, s) in split_data.iter().enumerate() {
            let val: u64 = s.parse().unwrap();
            data[i].push(val);
        }
    }

    for (op, args) in ops.iter().zip(data.iter()) {
        result += do_octopus_math(op, args.clone());
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::fs::File;
    use std::path::Path;

    #[test]
    fn get_total_rolls_example_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test1.txt");
        let data = File::open(path).expect("test1.txt file missing");
        let result = get_total_octopus_result(data);
        assert_eq!(result, 4277556);
    }

    #[test]
    fn get_total_rolls_real_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/input.txt");
        let data = File::open(path).expect("input.txt file missing");
        let result = get_total_octopus_result(data);
        assert_eq!(result, 4951502530386);
    }
}
