pub fn generate_random_odd_integer() -> u64 {
    rand::random::<u64>() | 1 | 1u64 << 63
}

pub fn generate_random_prime() -> u64 {
    let small_primes: &[u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    loop {
        let n = generate_random_odd_integer();
        if small_primes.iter().any(|&p| n.rem_euclid(p) == 0) {
            continue;
        }
    }
}
