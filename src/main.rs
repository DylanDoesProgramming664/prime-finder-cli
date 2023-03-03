#![allow(unused_imports, dead_code)]

pub mod prime_math;
pub mod prime_table;

use std::process::exit;

use crate::prime_table::PrimeTable;
use reedline::{DefaultPrompt, DefaultPromptSegment, Reedline, Signal};

fn main() {
    println!("Type an unsigned integer in the prompt to check for all primes up to that number.");
    let mut reader = Reedline::create();
    let prompt = DefaultPrompt::new(
        DefaultPromptSegment::Basic("input".to_owned()),
        DefaultPromptSegment::Empty,
    );

    loop {
        let sig = reader.read_line(&prompt);
        match sig {
            Ok(Signal::Success(buffer)) => {
                let num: u32 = if let Ok(num) = buffer.parse::<u32>() {
                    num
                } else {
                    println!("Input is not an unsigned integer");
                    continue;
                };
                let mut prime_table = PrimeTable::load();
                {
                    let this = &mut prime_table;
                    let prev_primes = this.stored_primes.clone();
                    if prev_primes.is_empty() {
                        (1..=num)
                            .filter(|uint| prime_math::is_prime(*uint))
                            .for_each(|new_prime| this.stored_primes.push(new_prime));
                        return;
                    }
                    (1..=num)
                        .filter(|uint| prime_math::is_prime(*uint))
                        .filter(|prime| !prev_primes.contains(prime))
                        .for_each(|new_prime| this.stored_primes.push(new_prime));
                };
                println!(
                    "Primes below {num}: {:?}",
                    prime_table.get_primes_in_range(num)
                );
                prime_table.save();
            }
            Ok(Signal::CtrlD | Signal::CtrlC) => {
                println!("\nAborted!");
                break;
            }
            Err(x) => {
                println!("Error: {x:?}");
            }
        }
    }
}
