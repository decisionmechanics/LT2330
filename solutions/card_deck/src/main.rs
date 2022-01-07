use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashMap;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Copy, Clone, EnumIter)]
enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Value {
    Number(u32),
    Jack,
    Queen,
    King,
    Ace,
}

#[allow(dead_code)]
struct Card {
    suit: Suit,
    value: Value,
}

fn create_deck() -> Vec<Card> {
    let mut deck = Vec::new();

    for suit in Suit::iter() {
        for i in 2..=10 {
            deck.push(Card {
                suit,
                value: Value::Number(i),
            });
        }
    }

    for suit in Suit::iter() {
        deck.push(Card {
            suit,
            value: Value::Jack,
        });

        deck.push(Card {
            suit,
            value: Value::Queen,
        });

        deck.push(Card {
            suit,
            value: Value::King,
        });

        deck.push(Card {
            suit,
            value: Value::Ace,
        });
    }

    deck
}

fn shuffle_deck(deck: &mut [Card]) {
    deck.shuffle(&mut thread_rng());
}

fn deal_hand(mut deck: Vec<Card>) -> (Vec<Card>, Vec<Card>) {
    let mut hand = Vec::new();

    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    (hand, deck)
}

fn has_pair(hand: &[Card]) -> bool {
    let mut counts = HashMap::new();

    for card in hand {
        *counts.entry(card.value).or_insert(0) += 1;
    }

    for (_, count) in counts {
        if count >= 2 {
            return true;
        }
    }

    false
}

fn has_full_house(hand: &[Card]) -> bool {
    let mut counts = HashMap::new();

    for card in hand {
        *counts.entry(card.value).or_insert(0) += 1;
    }

    let mut card_totals: Vec<i32> = counts.values().cloned().collect();
    card_totals.sort();

    card_totals[0] == 2 && card_totals[1] == 3
}

fn main() {
    let mut deck = create_deck();

    shuffle_deck(deck.as_mut_slice());

    let (hand, deck) = deal_hand(deck);

    println!("# cards in hand is {}", hand.len());
    println!("Pair? {}", has_pair(hand.as_slice()));
    println!("Full house? {}", has_full_house(hand.as_slice()));
    println!("# cards remaining in deck is {}", deck.len());
}
