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
    println!("select Hiragana (h) or Katakana (k)");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let kana;
    if input.trim().to_lowercase() == "h" {
        println!("Hiragana selected. Ctrl+C to exit anytime, answer last question before.");
        kana = kana::hiragana::hiragana();
    } else if input.trim().to_lowercase() == "k" {
        println!("Katakana selected. Ctrl+C to exit anytime, answer last question before.");
        kana = kana::katakana::katakana();
    } else {
        println!("Invalid input");
        return;
    }

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .unwrap();

    let mut rng = rand::rng();

    let mut successes = 0;
    let mut errors = 0;
    let mut total = 0;

    while running.load(Ordering::SeqCst) {
        clearscreen::clear().unwrap();
        io::stdout().flush().unwrap();

        let kana_item = kana.choose(&mut rng).unwrap();

        println!("How do you read?");
        println!("{}", kana_item.character);

        let start = Instant::now();

        println!("Romaji >");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            break;
        }

        let elapsed = start.elapsed();

        let input = input.trim().to_lowercase();
        total += 1;

        if kana_item.romaji.iter().any(|r| *r == input) || input == kana_item.written {
            println!("Correct!");
            successes += 1;
        } else {
            println!(
                "Incorrect. The correct answer is written '{}' and in romanji '{}'",
                kana_item.written,
                kana_item.romaji.join(", ")
            );
            errors += 1;
        }

        println!("\nTime taken: {:.2?} secs", elapsed.as_secs());
        thread::sleep(Duration::from_secs(4));
    }

    clearscreen::clear().unwrap();
    println!("\n===== RESULTS =====");
    println!("Total questions: {total}");
    println!("Successes: {successes}");
    println!("Errors: {errors}");

    let percentaje = if total > 0 {
        (successes as f64 / total as f64) * 100.0
    } else {
        0.0
    };

    println!("Overall Knowledge: {:.2}%", percentaje);
}
