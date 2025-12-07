use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq)]
pub enum OctopusOperator {
    Add,
    Multiply,
}

/// do an octopus math problem and return the result
/// octopus math involves applying the same operator to a bunch of arguments and returning the aggregated result
///
/// # Examples
/// ```
/// let result = day06::do_octopus_math(day06::OctopusOperator::Multiply, vec![123, 45, 6]);
/// assert_eq!(result, 33210);
/// ```
/// ```
/// let result = day06::do_octopus_math(day06::OctopusOperator::Add, vec![328, 64, 98]);
/// assert_eq!(result, 490);
/// ```
pub fn do_octopus_math(op: OctopusOperator, args: Vec<u64>) -> u64 {
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

/// convert a problem from a matrix of chars to a tuple of operator and vector of numbers
///
/// # Examples
///
/// ```
/// let input = vec![vec![' ', '4', '6', ' '], vec![' ', '3', '2', ' '], vec!['4', '1', '3', ' '], vec![' ', ' ', '+', ' ']];
/// let (op, args) = day06::convert_problem(input);
/// assert_eq!(op, day06::OctopusOperator::Add);
/// assert_eq!(args, vec![4,431,623]);
/// ```
pub fn convert_problem(problem: Vec<Vec<char>>) -> (OctopusOperator, Vec<u64>) {
    let mut result: Vec<u64> = Vec::new();
    let copied = problem.clone();
    let raw_ops: &Vec<char> = copied.iter().last().unwrap();
    let mut ops: Vec<OctopusOperator> = Vec::new();

    for op in raw_ops.clone() {
        match op {
            '+' => ops.push(OctopusOperator::Add),
            '*' => ops.push(OctopusOperator::Multiply),
            ' ' => continue,
            other => panic!("weird operator found - {other}"),
        }
    }
    let mut raw_args: Vec<String> = Vec::new();
    let height: usize = copied.len();
    for i in 0..height - 1 {
        let width: usize = copied[i].len();
        for j in 0..width {
            if raw_args.len() <= j {
                raw_args.push(String::new())
            }
            let arg = copied[i][j];
            if arg == ' ' {
                continue;
            }
            raw_args[j].push(arg);
        }
    }
    //now raw_args has a bunch of strings which should parse to numbers for result
    println!("raw_args: {raw_args:?}");
    for arg in raw_args {
        if arg.is_empty() {
            continue;
        }
        let num: u64 = arg.parse().unwrap();
        result.push(num);
    }

    (ops.pop().unwrap(), result)
}

pub fn get_total_octopus_result(input: File) -> u64 {
    let mut result: u64 = 0;
    let buf = BufReader::new(input);
    let mut data: Vec<Vec<char>> = Vec::new();

    //get all the input data into a huge char matrix
    for line in buf.lines() {
        let validated_line = line.expect("weird line");
        let split_data: Vec<char> = validated_line.chars().collect();
        data.push(split_data);
    }
    'matrix: loop {
        if data[0].is_empty() {
            break 'matrix;
        }

        let mut blanks: usize;

        //slice each specific problem from the rest
        let problem_size = data.len();
        let mut current: Vec<Vec<char>> = Vec::new();
        for _ in 0..problem_size {
            let new_vec: Vec<char> = Vec::new();
            current.push(new_vec);
        }
        'problem: loop {
            blanks = 0;
            for j in 0..problem_size {
                //println!("problem loop {j}. current is {current:?}");
                if data[j].is_empty() {
                    //if there's no data left, that's the end of the problem
                    //println!("problem over, no data left. current is {current:?}");
                    break 'problem;
                }
                let cell = data[j].pop().unwrap();
                if cell == ' ' {
                    blanks += 1;
                }
                current[j].push(cell);
            }
            //println!("problem over, found {blanks} blanks");
            if blanks == problem_size {
                //we have a fully blank column, current problem is done;
                break 'problem;
            }
        }

        //convert current problem section to the Vec<u64> and operator
        let (op, args) = convert_problem(current.clone());

        //and now get the result
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
        assert_eq!(result, 3263827);
    }

    #[test]
    fn get_total_rolls_real_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/input.txt");
        let data = File::open(path).expect("input.txt file missing");
        let result = get_total_octopus_result(data);
        assert_eq!(result, 8486156119946);
    }
}
