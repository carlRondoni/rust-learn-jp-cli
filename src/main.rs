use rand::{self, seq::IndexedRandom};
use rust_learn_jp_cli::kana;
use std::{
    io::{self, Write},
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

fn main() {
    clearscreen::clear().unwrap();
    println!("Welcome to rust-learn-jp-cli");
    println!("Please select mode: k (train kana)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    if input.trim().to_lowercase() == "k" {
        kana::kana_mode::start();
    } else {
        println!("invalid mode");
        return;
    }

    println!("thanks for using this CLI!");
}
