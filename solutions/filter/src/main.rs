fn filter<T: Copy>(pred: fn(T) -> bool, items: &[T]) -> Vec<T> {
    let mut result = Vec::new();

    for &item in items {
        if pred(item) {
            result.push(item);
        }
    }

    result
}

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }

    let mut maximum_possible_divisor = (n as f64).sqrt() as u64;

    if maximum_possible_divisor < 2 {
        maximum_possible_divisor = 2;
    }

    for i in 2..=maximum_possible_divisor {
        if n % i == 0 {
            return false;
        }
    }

    true
}

fn main() {
    let numbers = (1u64..=100).collect::<Vec<_>>();

    let primes = filter(is_prime, numbers.as_slice());

    for prime in primes {
        println!("{}", prime);
    }
}
