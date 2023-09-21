// Rust lab1
// A01243888 Nash Baek

use std::collections::HashMap;

// Function to find primes between m and n
fn primes(m: usize, n: usize) -> Vec<usize> {
    // Initialize all elements as true which means it is prime number
    // After that, change each elements into false when it is determined as non-prime number.
    let mut sieve = Vec::new();
    for _ in 0..n {
        sieve.push(true);
    }

    // 0 and 1 are non-prime numbers
    sieve[0] = false;
    sieve[1] = false;

    // Iterate through numbers up to the square root of n
    // We only need to check numbers up to sqrt(n) to identify all non-prime numbers
    for p in 2..=((n as f64).sqrt() as usize) {
        // If p is a prime number (not marked as false)
        if sieve[p] {
            // Mark all multiples of p as non-prime
            // We start from p*p because all smaller multiples of p have already been marked by smaller primes
            for multiple in (p * p..n).step_by(p) {
                sieve[multiple] = false;
            }
        }
    }

    // Initialize an empty vector to store the prime numbers
    let mut primes = Vec::new();
    // Iterate through the sieve array to collect the prime numbers
    // We start from index m and go up to n
    // The enumerate() function provides the index as well as the value (true or false)
    for (i, &is_prime) in sieve.iter().enumerate().skip(m).take(n - m) {
        // If the number is prime (marked as true)
        if is_prime {
            // Add it to the list of prime numbers
            primes.push(i);
        }
    }

    primes
}

fn main() {
    let prime_list = primes(100_000, 1_000_000); // 6-digit primes
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();

    // Grouping primes by their sorted digits as the key
    // Iterate through each prime number in the prime_list
    // The "&" is for borrowing the value without taking ownership
    for &prime in &prime_list {
        // Convert the prime number to a string and then collect its characters into a vector
        let mut digits: Vec<char> = prime.to_string().chars().collect();
        // Sort the digits
        digits.sort();
        // Convert the sorted digits back to a string to use as a key in the HashMap
        let key: String = digits.into_iter().collect();
        // Insert the prime number into the HashMap under the sorted digits key
        // If the key doesn't exist, a new vector is created and the prime is pushed into it
        groups.entry(key).or_insert(Vec::new()).push(prime);
    }

    let mut max_size = 0;
    let mut largest_group = Vec::new();

    // Finding the largest group of primes that are permutations of one another
    for group in groups.values() {
        if group.len() > max_size {
            max_size = group.len();
            largest_group = group.clone();
        }
    }

    println!(
        "The size of the largest set of 6-digit primes that are permutations of one another is: {}",
        max_size
    );
    println!("The primes in this largest set are: {:?}", largest_group);
}
