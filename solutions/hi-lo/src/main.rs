use rand::Rng;
use std::io;

fn get_secret() -> u32 {
    rand::thread_rng().gen_range(1..101)
}

#[derive(PartialEq)]
enum GuessStatus {
    Uninitialized,
    TooLow,
    TooHigh,
    Exact,
}

struct HiLo {
    secret: u32,
    attempts: u32,
}

impl HiLo {
    fn new() -> HiLo {
        let hi_lo = HiLo {
            secret: get_secret(),
            attempts: 0,
        };

        hi_lo
    }

    fn make_guess(&mut self, guess: u32) -> GuessStatus {
        self.attempts += 1;

        if guess < self.secret {
            GuessStatus::TooLow
        } else if guess > self.secret {
            GuessStatus::TooHigh
        } else {
            GuessStatus::Exact
        }
    }
}

fn read_guess() -> u32 {
    println!("What number between 1 and 100 am I thinking of?");

    let mut s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read from input");

    s.trim().parse::<u32>().unwrap()
}

fn main() {
    let mut hi_lo = HiLo::new();

    let mut guess_status = GuessStatus::Uninitialized;

    while guess_status != GuessStatus::Exact {
        let guess = read_guess();

        guess_status = hi_lo.make_guess(guess);

        match guess_status {
            GuessStatus::TooLow => println!("Awww... {} is too low.", guess),
            GuessStatus::TooHigh => println!("Steady... {} is too high!", guess),
            _ => println!(
                "Congratulations! You took {} attempt(s) to guess I was thinking of {}.",
                hi_lo.attempts, hi_lo.secret
            ),
        }
    }
}
