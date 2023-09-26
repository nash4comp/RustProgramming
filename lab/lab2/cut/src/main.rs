use std::io::{self, BufRead};
use std::env;

enum Range {
    Single(usize),
    FromStart(usize),
    ToEnd(usize),
    Between(usize, usize),
}

impl Range {
    fn parse(s: &str) -> Option<Range> {
        let parts: Vec<&str> = s.split('-').collect();
        match parts.len() {
            1 => parts[0].parse().ok().map(Range::Single),
            2 => {
                if parts[0].is_empty() {
                    parts[1].parse().ok().map(Range::FromStart)
                } else if parts[1].is_empty() {
                    parts[0].parse().ok().map(Range::ToEnd)
                } else {
                    let start = parts[0].parse().ok()?;
                    let end = parts[1].parse().ok()?;
                    if start <= end {
                        Some(Range::Between(start, end))
                    } else {
                        None
                    }
                }
            }
            _ => None,
        }
    }

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
        eprintln!("Usage: ./cut <ranges>");
        return Ok(());
    }

    let ranges: Vec<Range> = args[1]
        .split(',')
        .filter_map(|s| Range::parse(s))
        .collect();

    if ranges.is_empty() {
        eprintln!("Invalid range input.");
        return Ok(());
    }

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let line = line?;
        let mut output = String::new();
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
