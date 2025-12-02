use std::{
    io::BufRead,
    io::BufReader,
    fs::File,
    path::Path
};

pub fn rotate(start: u32, clicks: u32, direction: char) -> (u32, u32) {
    let mut position:i32 = start as i32;
    let mut zeroes:u32 = 0;
    //we can discard batches of 100 clicks since they end up at same place on dial
    let relevant_clicks = clicks;
    /*while relevant_clicks > 100 {
        relevant_clicks = relevant_clicks - 100;
        zeroes += 1;
    }*/

    if direction == 'R' {
        // turning right, going higher, might pass 99
        for _ in 0..= relevant_clicks - 1 {
            if position == 99 {
                position = -1;
                zeroes += 1;
            }
            position += 1;
        }
    } else if direction == 'L' {
        //turning left, going lower, might pass 0
        for _ in 0..= relevant_clicks - 1 {
            if position == 0 {
                position = 100;
                zeroes += 1;
            }
            position -= 1;
        }
    } else {
        panic!("invalid direction {direction}")
    }
    println!("{direction} to position {position} and encountered {zeroes} zeroes");
    (position as u32, zeroes)
}

pub fn parse_input_line(input: &str) -> (char, u32){
    let (raw_dir, raw_clicks) = input.split_at(1);
    let direction = raw_dir.chars().last().expect("Missing direction");
    let clicks:u32 = raw_clicks.parse().expect("unable to parse clicks from line");
    (direction, clicks)
}

pub fn check_safe(input: File) -> (u32, u32) {
    let mut dial:u32 = 50;
    let mut rests_at_zero:u32 = 0;
    let mut extra_zeroes:u32;
    let buf = BufReader::new(input);
    for line in buf.lines() {
        let (direction, clicks) = parse_input_line(&line.expect("weird line"));
        (dial, extra_zeroes) = rotate(dial, clicks, direction);
        rests_at_zero += extra_zeroes;
    }

    (rests_at_zero, dial)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_small_right_works() {
        let result = rotate(50, 13, 'R');
        assert_eq!(result, (63, 0));
    }

    #[test]
    fn rotate_big_right_works() {
        let result = rotate(50, 10063, 'R');
        assert_eq!(result, (13, 101));
    }

    #[test]
    fn rotate_small_left_works() {
        let result = rotate(50, 12, 'L');
        assert_eq!(result, (38, 0));
    }

    #[test]
    fn rotate_big_left_works() {
        let result = rotate(50, 10045, 'L');
        assert_eq!(result, (5, 100));
    }

    #[test]
    fn rotate_big_left_overflow_works() {
        let result = rotate(50, 10055, 'L');
        assert_eq!(result, (95, 101));
    }

    #[test]
    #[should_panic]
    fn rotate_panics_on_weird_direction() {
        rotate(50, 4, 'X');
    }

    #[test]
    fn parse_input_line_handles_left() {
        let result = parse_input_line("L2000");
        assert_eq!(result, ('L', 2000));
    }

    #[test]
    fn parse_input_line_handles_right() {
        let result = parse_input_line("R15");
        assert_eq!(result, ('R', 15));
    }

    #[test]
    fn check_safe_example_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test1.txt");
        let data = File::open(path).expect("test1.txt file missing");
        let result = check_safe(data);
        assert_eq!(result, (6, 32));
    }

    #[test]
    fn check_safe_example_2_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/test2.txt");
        let data = File::open(path).expect("test1.txt file missing");
        let result = check_safe(data);
        assert_eq!(result, (10, 50));
    }

    #[test]
    fn check_safe_real_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/input.txt");
        let data = File::open(path).expect("test1.txt file missing");
        let result = check_safe(data);
        assert_eq!(result, (5956, 97));
    }


}
