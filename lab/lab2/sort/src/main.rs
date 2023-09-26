// Rust Lab 2-1
// Nash Baek, A01243888

// Criteria
// 1. Descending order of scores.
// 2. If same above, ascending order of last names
// 3. If same above, ascending order of first names

// Import necessary modules
use std::env;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

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
    // Collect command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: ./sort <filename>");
        return Ok(());
    }

    // Get the filename from command line arguments
    let filename = &args[1];

    // Open the file
    let file = File::open(filename)?;

    // Create a buffered reader for the file
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

        // Create a record from parsed data
        let record = Record {
            first_name: words[0].to_string(),
            last_name: words[1].to_string(),
            score,
        };

        // Push the record into the vector
        records.push(record);
    }

    // Sort the records in descending order based on the score,
    // and in case of a tie, sort by last name, and then by first name.
    records.sort_by(|a, b| {
        // Compare the scores in descending order (higher scores first)
        // using the `cmp` method.
        b.score
            .cmp(&a.score)
            // If scores are equal, compare by last names in ascending order.
            // This sorts names alphabetically from A to Z.
            .then(a.last_name.cmp(&b.last_name))
            // If last names are also equal, compare by first names in ascending order.
            .then(a.first_name.cmp(&b.first_name))
    });

    // Display the sorted records
    for record in records {
        println!("{}", record);
    }

    Ok(())
}
