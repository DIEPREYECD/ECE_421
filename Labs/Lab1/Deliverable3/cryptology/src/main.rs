use apint::ApInt;
use primes::is_prime;
use rand::prelude::*;

fn main() {
    // println!("Hello, world!");
    // let prime = fucntion_q2(1000);
    // println!("Found prime: {}", prime.try_to_u64().unwrap());

    prime_fun(100);
}

fn fucntion_q2(n: u32) -> ApInt {
    let mut rng = rand::thread_rng();
    loop {
        let rand_uint = rng.gen_range(0, n);

        let mut candidate = ApInt::from_u32(rand_uint);
        let _ = candidate.set_bit_at(0);

        if is_prime(candidate.try_to_u64().unwrap()) {
            return candidate;
        }
    }
}


fn prime_fun(sum_limit: u64) {
    // Generate a list of prime numbers below sum_limit
    let primes: Vec<u64> = (2..sum_limit).filter(|x| is_prime(*x)).collect();

    /*
    My approach is to use a sliding window to find the longest sequence of consecutive primes that sum to a prime number below sum_limit.
    I will stop when I find the longest sequence, which is when I attempt to find a sequence witha longer window size than the current longest sequence that 
    satisfies the conditions but I fail to find any such sequence.
     */

    let mut longest_sequence: Vec<u64> = Vec::new();
    let mut max_length = 0;

    for window_size in (1..primes.len()).rev() {
        let mut found = false;

        // Go through the primes with the current window size and check sums
        for start in 0..=primes.len() - window_size {
            let sequence: Vec<u64> = primes[start..start + window_size].to_vec();
            let sum: u64 = sequence.iter().sum();
            if sum >= sum_limit {
                break;
            }
            if is_prime(sum) {
                longest_sequence = sequence;
                max_length = window_size;
                found = true;
                break;
            }
        }
        if found {
            break;
        }
    }

    
    // Print results
    println!("X (length of sequence): {}", max_length);
    let sum: u64 = longest_sequence.iter().sum();
    println!("Y (sum of sequence): {}", sum);
    println!("List of primes: {:?}", longest_sequence);

}