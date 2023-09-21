use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::env;

fn main() -> io::Result<()> {
    // Fetch the filename from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: cargo run <filename>");
        std::process::exit(1);
    }

    // Open the file
    let filename = &args[1];
    let file = File::open(filename)?;

    // Use BufReader module for better performance
    let reader = BufReader::new(file);

    // HashMap to store the frequency of each word
    let mut word_count: HashMap<String, u32> = HashMap::new();
    
    // Loop through each line in the file
    for line in reader.lines() {
        // When the line from the buffer has valid value, it will store the value
        // else it will occur error message using ? operator
        let line = line?;
        
        // Split the line into words and iterate through each word
        for word in line.split_whitespace() {
            let word = word.to_lowercase();
            
            // Increase the word count
            *word_count.entry(word).or_insert(0) += 1;
        }
    }
    
    // Find the maximum frequency
    let mut max_count: u32 = 0;
    for &count in word_count.values() {
        if count > max_count {
            max_count = count;
        }
    }

    // Print words that have the maximum frequency
    for (word, count) in &word_count {
        if *count == max_count {
            println!("{}", word);
        }
    }

    Ok(())
}