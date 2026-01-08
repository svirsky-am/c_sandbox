use std::fmt;

// Define an enum
enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

// Implement the Display trait for the Suit enum
impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Suit::Heart => write!(f, "♥"),
            Suit::Diamond => write!(f, "♦"),
            Suit::Spade => write!(f, "♠"),
            Suit::Club => write!(f, "♣"),
        }
    }
}

fn main() {
    let heart = Suit::Heart;
    let diamond = Suit::Diamond;

    // Print the enum values using the Display implementation
    println!("My favorite suit is {}", heart);
    println!("Another suit: {}", diamond);
}