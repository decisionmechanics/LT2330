use rand::Rng;

#[derive(PartialEq)]
pub enum GuessStatus {
    Uninitialized,
    TooLow,
    TooHigh,
    Exact,
}

pub struct HiLo {
    pub secret: i32,
    pub attempts: i32,
    last_guess: Option<i32>,
}

impl HiLo {
    pub fn new() -> HiLo {
        let mut hi_lo = HiLo {
            secret: 0,
            attempts: 0,
            last_guess: None,
        };

        hi_lo.reset();

        hi_lo
    }

    pub fn reset(&mut self) {
        self.secret = rand::thread_rng().gen_range(1..101);
        self.attempts = 0;
        self.last_guess = None;
    }

    pub fn make_guess(&mut self, guess: i32) -> GuessStatus {
        self.attempts += 1;
        self.last_guess = Some(guess);

        if guess < self.secret {
            GuessStatus::TooLow
        } else if guess > self.secret {
            GuessStatus::TooHigh
        } else {
            GuessStatus::Exact
        }
    }
}
