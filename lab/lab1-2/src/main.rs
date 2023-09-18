use std::collections::HashMap;

// m과 n 사이의 소수를 찾는 함수
fn primes(m: usize, n: usize) -> Vec<usize> {
    let mut sieve = vec![true; n];
    sieve[0] = false;
    sieve[1] = false;
    
    for p in 2..=((n as f64).sqrt() as usize) {
        if sieve[p] {
            for multiple in (p * p..n).step_by(p) {
                sieve[multiple] = false;
            }
        }
    }

    let mut primes = Vec::new();
    for (i, &is_prime) in sieve.iter().enumerate().take(n).skip(m) {
        if is_prime {
            primes.push(i);
        }
    }

    primes
}

fn main() {
    let prime_list = primes(100_000, 1_000_000); // 6자리 소수들
    let mut groups: HashMap<String, Vec<usize>> = HashMap::new();

    // 소수들을 숫자별로 정렬한 문자열을 키로 사용하여 그룹화
    for &prime in &prime_list {
        let mut digits: Vec<char> = prime.to_string().chars().collect();
        digits.sort();
        let key: String = digits.into_iter().collect();
        groups.entry(key).or_insert(Vec::new()).push(prime);
    }

    let mut max_size = 0;
    let mut largest_group = Vec::new();
    
    for group in groups.values() {
        if group.len() > max_size {
            max_size = group.len();
            largest_group = group.clone();
        }
    }

    println!("The size of the largest set of 6-digit primes that are permutations of one another is: {}", max_size);
    println!("The primes in this largest set are: {:?}", largest_group);
}
