use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;

pub fn rotate(start: u32, clicks: u32, direction: char) -> u32 {
    let mut position:i32;
    //we can discard batches of 100 clicks since they end up at same place on dial
    let mut relevant_clicks = clicks;
    while relevant_clicks > 100 {
        relevant_clicks = relevant_clicks - 100;
    }

    if direction == 'R' {
        // turning right, going higher, might pass 99
        position = (start + relevant_clicks) as i32;
        if position > 99 {
            position = position - 100;
        }
    } else if direction == 'L' {
        //turning left, going lower, might pass 0
        position = (start) as i32 - (relevant_clicks) as i32;
        if position < 0 {
            position = position + 100;
        }
    } else {
        panic!("invalid direction {direction}")
    }
    position as u32
}

pub fn parse_input_line(input: &str) -> (char, u32){
    let (raw_dir, raw_clicks) = input.split_at(1);
    let direction = raw_dir.chars().last().expect("Missing direction");
    let clicks:u32 = raw_clicks.parse().expect("unable to parse clicks from line");
    (direction, clicks)
}

pub fn check_safe(input: File) -> u32 {
    let mut dial:u32 = 50;
    let mut rests_at_zero:u32 = 0;
    let buf = BufReader::new(input);
    for line in buf.lines() {
        let (direction, clicks) = parse_input_line(&line.expect("weird line"));
        dial = rotate(dial, clicks, direction);
        if dial == 0 {
            rests_at_zero += 1;
        }
    }
    rests_at_zero
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_small_right_works() {
        let result = rotate(50, 13, 'R');
        assert_eq!(result, 63);
    }

    #[test]
    fn rotate_big_right_works() {
        let result = rotate(50, 10063, 'R');
        assert_eq!(result, 13);
    }

    #[test]
    fn rotate_small_left_works() {
        let result = rotate(50, 12, 'L');
        assert_eq!(result, 38);
    }

    #[test]
    fn rotate_big_left_works() {
        let result = rotate(50, 10045, 'L');
        assert_eq!(result, 5);
    }

    #[test]
    fn rotate_big_left_overflow_works() {
        let result = rotate(50, 10055, 'L');
        assert_eq!(result, 95);
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
        assert_eq!(result, 3);
    }

    #[test]
    fn check_safe_real_works() {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("src/input.txt");
        let data = File::open(path).expect("test1.txt file missing");
        let result = check_safe(data);
        assert_eq!(result, 1043);
    }


}
