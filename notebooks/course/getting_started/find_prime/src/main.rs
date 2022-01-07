fn is_prime(n: u64) -> bool {
    let maximum = (n as f64).sqrt() as u64;

    for i in 2..=maximum {
        if n % i == 0 {
            return false;
        }
    }

    return true;
}

fn find_nth_prime(n: u32) -> u64 {
    let mut count = n;
    let mut candidate = 1u64;

    while count > 0 {
        candidate += 1;

        if is_prime(candidate) {
            count -= 1;
        }
    }

    return candidate;
}

fn get_number_suffix(n: u32) -> String {
    return match n {
        11..=13 => "th",
        _ if n % 10 == 1 => "st",
        _ if n % 10 == 2 => "nd",
        _ if n % 3 == 0 => "rd",
        _ => "th",
    }
    .to_string();
}

fn main() {
    const N: u32 = 1_000;

    let prime = find_nth_prime(N);
    let number_suffix = get_number_suffix(N);

    println!("The {}{} prime is {}.", N, number_suffix, prime);
}
