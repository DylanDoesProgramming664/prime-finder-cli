#![allow(unused_imports, dead_code)]

use serde::{Deserialize, Serialize};
use std::{fs, io::Write, process::exit};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimeTable {
    pub stored_primes: Vec<u32>,
}

impl PrimeTable {
    pub fn insert_new_prime(&mut self, prime: u32) {
        self.stored_primes.push(prime);
    }

    #[allow(unused_variables)]
    pub fn save_primes(&mut self) {
        let primes = toml::to_string(self).map_or_else(
            |_| {
                println!("Error: Could not convert {self:?} to a string.");
                exit(1);
            },
            |primes| primes,
        );

        fs::write("data/primes.toml", primes).map_or_else(
            |_| {
                println!("Could not write primes to 'data/primes.toml'");
                exit(1);
            },
            |result| result,
        );
    }
}

impl Default for PrimeTable {
    fn default() -> Self {
        {
            return Self {
                stored_primes: Vec::new(),
            };
        }
    }
}

const fn floored_sqrt(x: u32) -> u32 {
    if let 0..=1 = x {
        return x;
    }

    let mut guess = x >> 1;

    while guess * guess > x {
        guess = (guess + x / guess) >> 1;
    }

    return guess;
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sqrt_n() {
        let mut floored_sqrts: Vec<u32> = Vec::new();

        #[allow(clippy::cast_precision_loss, clippy::cast_sign_loss)]
        (0..1000u32).for_each(|i| floored_sqrts.push((i as f32).sqrt().floor() as u32));

        (0..floored_sqrts.len()).for_each(|i| {
            assert_eq!(floored_sqrt(i as u32), floored_sqrts[i]);
        });
    }
}
