#![allow(unused_imports, dead_code)]

pub mod prime_math;
pub mod prime_table;

use std::{fs, process::exit};

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
                let num: u64 = if let Ok(num) = buffer.parse::<u64>() {
                    num
                } else {
                    println!("Input is not an unsigned integer");
                    continue;
                };
                let mut prime_table = PrimeTable::load();
                prime_table.generate_primes(num);
                println!(
                    "Primes up to {num}: {:?}",
                    &prime_table.get_primes_in_range(num)
                );
                println!(
                    "Num primes up to {num}: {:?}",
                    prime_table.get_primes_in_range(num).len()
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
