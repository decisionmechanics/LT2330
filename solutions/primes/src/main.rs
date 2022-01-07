fn is_prime(n: u64) -> bool {
    let maximum_possible_divisor = (n as f64).sqrt() as u64;

    for i in 2..=maximum_possible_divisor {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    const MAXIMUM_VALUE: u64 = 100;

    let mut primes_found = 0;

    for i in 2..=MAXIMUM_VALUE {
        if is_prime(i) {
            primes_found += 1;

            println!("{} (#{})", i, primes_found);
        }
    }
}
