use std::io;

//mod game;

fn read_guess() -> i32 {
    println!("What number between 1 and 100 am I thinking of?");

    let mut s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read from input");

    s.trim().parse().unwrap()
}

fn main() {
    let mut hi_lo = game::HiLo::new();

    let mut guess_status = game::GuessStatus::Uninitialized;

    while guess_status != game::GuessStatus::Exact {
        let guess = read_guess();

        guess_status = hi_lo.make_guess(guess);

        match guess_status {
            game::GuessStatus::TooLow => println!("Awww... {} is too low.", guess),
            game::GuessStatus::TooHigh => println!("Steady... {} is too high!", guess),
            _ => (),
        }
    }

    println!(
        "Congratulations! You took {} attempt(s) to guess I was thinking of {}.",
        hi_lo.attempts, hi_lo.secret
    );
}
