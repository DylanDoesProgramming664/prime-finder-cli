use serde::{Deserialize, Serialize};
use std::{
    collections, env, fs,
    io::{self, Write},
    process::exit,
};

use crate::prime_math;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrimeTable {
    pub stored_primes: Vec<u64>,
}

impl PrimeTable {
    const fn new() -> Self {
        return Self {
            stored_primes: Vec::new(),
        };
    }

    #[must_use]
    pub fn from_vec(vec: Vec<u64>) -> Self {
        return Self {
            stored_primes: Vec::from_iter(vec),
        };
    }

    pub fn save(&mut self) {
        let primes = toml::to_string(self).map_or_else(
            |x| {
                println!("Error: {x:?}");
                exit(1);
            },
            |primes| primes,
        );

        fs::write("data/primes.toml", primes).map_or_else(
            |_| {
                let _data_path = fs::create_dir("data");
                self.save();
            },
            |result| result,
        );
    }

    #[must_use]
    pub fn load() -> Self {
        let mut is_data_stored = true;
        let data = fs::read_to_string("data/primes.toml").map_or_else(
            |_| {
                println!("No prime table to fetch, generating new table.");
                is_data_stored = false;
                String::new()
            },
            |data| data,
        );
        if !is_data_stored {
            return Self::new();
        }
        return toml::from_str(&data).map_or_else(
            |_| {
                println!("Could not map data to a PrimeTable struct. Generating new PrimeTable.");
                Self::new()
            },
            |primes| primes,
        );
    }

    pub fn generate_primes(&mut self, num: u64) {
        let prev_primes = self.stored_primes.clone();

        if prev_primes.is_empty() {
            (1..=num)
                .filter(|uint| prime_math::is_prime(*uint, &prev_primes))
                .for_each(|new_prime| self.stored_primes.push(new_prime));
            return;
        }

        if num < prev_primes[prev_primes.len() - 1] {
            return;
        }

        (prev_primes[prev_primes.len() - 1]..=num)
            .filter(|uint| prime_math::is_prime(*uint, &prev_primes))
            .filter(|prime| !prev_primes.contains(prime))
            .for_each(|new_prime| self.stored_primes.push(new_prime));
    }

    pub fn get_primes_in_range(&mut self, input: u64) -> Vec<u64> {
        let output_vec = self
            .stored_primes
            .iter()
            .filter(|prime| **prime <= input)
            .copied()
            .collect::<Vec<u64>>();
        return output_vec;
    }
}
