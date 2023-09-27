// Rust Lab 2-2. Cut
// Nash Baek, A01243888

// Usage
// cargo run "2,-3,10,5-7,13-" < data.txt

use std::env;
use std::io::{self, BufRead};

enum Range {
    Single(usize),
    FromStart(usize),
    ToEnd(usize),
    Between(usize, usize),
}

impl Range {
    // Defines the parse function which takes a string reference &str as an argument and returns Option<Range> 
    // Option<Range> can either have a value of Range type or no value
    fn parse(s: &str) -> Option<Range> {
        // Splits the input string s based on the '-' character, storing the result in the parts variable
        let parts: Vec<&str> = s.split('-').collect();

        // Checks how many segments the string has been split into
        match parts.len() {
            // If the length of parts is 1, it tries to convert parts[0] to a number. If successful, it's transformed into Range::Single
            1 => parts[0].parse().ok().map(Range::Single),
            // Logic for when the length of parts is 2.
            2 => {
                // Checks if the first segment is empty
                // This logic matches patterns like -3, -5 where the range starts from 0 to a specific number
                if parts[0].is_empty() {
                    // If the first segment is empty, it tries to convert the second segment to a number. If successful, it's transformed into Range::FromStart
                    parts[1].parse().ok().map(Range::FromStart)
                }
                // Checks if the second segment is empty
                else if parts[1].is_empty() {
                    // If the second segment is empty, it tries to convert the first segment to a number. If successful, it's transformed into Range::ToEnd 
                    // This is for ranges starting from a specific number and going to the end of the line.
                    parts[0].parse().ok().map(Range::ToEnd)
                } 
                // Logic for when both segments are not empty
                else {
                    let start = parts[0].parse().ok()?;
                    let end = parts[1].parse().ok()?;
                    // Checks if start is less than or equal to end
                    if start <= end {
                        Some(Range::Between(start, end))
                    } else {
                        None
                    }
                }
            }
            // Returns None for other lengths of parts
            _ => None,
        }
    }

    // Checks if a given number matches the defined range
    fn contains(&self, n: usize) -> bool {
        match self {
            Range::Single(x) => *x == n,
            Range::FromStart(x) => n <= *x,
            Range::ToEnd(x) => n >= *x,
            Range::Between(x, y) => n >= *x && n <= *y,
        }
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: ./cut \"ranges\" <filename>");
        eprintln!("Example: ./cut \"2,-3,10,5-7,13-\" < data.txt");
        return Ok(());
    }

    // Reads the first argument, splits it by ',' and stores it in a vector of Range type called ranges
    let ranges: Vec<Range> = args[1].split(',').filter_map(|s| Range::parse(s)).collect();

    if ranges.is_empty() {
        eprintln!("Invalid range input.");
        return Ok(());
    }

    let stdin = io::stdin();
    // Lock function is used for synchronization
    for line in stdin.lock().lines() {
        let line = line?;
        let mut output = String::new();
        // Using the enumerate function, we get the index and character, which are assigned to i and ch respectively
        // During the iteration, ch is appended to output
        for (i, ch) in line.chars().enumerate() {
            if ranges.iter().any(|r| r.contains(i + 1)) {
                output.push(ch);
            }
        }
        output.push('\n');
        print!("{}", output);
    }
    Ok(())
}