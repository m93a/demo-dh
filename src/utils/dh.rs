use anyhow::Context;
use rand::distributions::Distribution;

trait ModPow {
    fn modpow(&self, exponent: &Self, modulus: &Self) -> Self;
}
impl ModPow for u64 {
    fn modpow(&self, &exponent: &u64, &modulus: &u64) -> u64 {
        let mut exponent = exponent;
        let mut base = *self;
        if modulus == 1 { return 0 }

        let mut result = 1;
        base = base % modulus;
        while exponent > 0 {
            if exponent % 2 == 1 {
                result = result * base % modulus;
            }
            exponent = exponent >> 1;
            base = base * base % modulus
        }
        result
    }
}


pub fn generate_random_odd_integer() -> u64 {
    (rand::random::<u32>() | 1).into()
}

pub fn generate_random_prime() -> u64 {
    let small_primes: &[u64] = &[2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31];
    loop {
        let n = generate_random_odd_integer();
        if small_primes.iter().any(|&p| n % p == 0) {
            continue;
        }
        if !is_probably_prime(n, 20) {
            continue;
        }
        return n;
    }
}

pub fn is_probably_prime(n: u64, test_count: usize) -> bool {
    // find m, k, such that n = m * 2ᵏ
    let mut k = 0u64;
    let mut m = n;
    while m % 2 == 0 {
        m >>= 1;
        k += 1;
    }
    let k = k;
    let m = m;

    let mut rng = rand::distributions::Uniform::new(1, n).sample_iter(rand::thread_rng());

    // perform the Rabin-Miller test
    'tests: for _ in 0..test_count {
        let tester = rng.next().unwrap();

        if tester.modpow(&m, &n) == 1 {
            continue 'tests;
        }

        for i in 0..k {
            if tester.modpow(&((1 << i) * m), &n) == n - 1 {
                continue 'tests;
            }
        }

        return false;
    }

    true
}
