use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;

// Define a struct to hold a record
struct Record {
    first_name: String,
    last_name: String,
    score: i32,
}

// Implement the Display trait for the Record struct
impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.score, self.last_name, self.first_name)
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./sort <filename>");
        return Ok(());
    }

    let filename = &args[1];
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    let mut records: Vec<Record> = Vec::new();

    // Read each line from the file
    for line in reader.lines() {
        let line = line?;
        let words: Vec<&str> = line.split_whitespace().collect();

        if words.len() < 3 {
            continue;
        }

        // Parse the score and ensure it's an integer between 0 and 100
        let score: i32 = match words[2].parse() {
            Ok(n) if n >= 0 && n <= 100 => n,
            _ => continue,
        };

        let record = Record {
            first_name: words[0].to_string(),
            last_name: words[1].to_string(),
            score,
        };

        records.push(record);
    }

    // Sort the records
    records.sort_by(|a, b| {
        b.score.cmp(&a.score)
            .then(a.last_name.cmp(&b.last_name))
            .then(a.first_name.cmp(&b.first_name))
    });

    // Display the sorted records
    for record in records {
        println!("{}", record);
    }

    Ok(())
}
