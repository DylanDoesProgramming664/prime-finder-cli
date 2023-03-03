use serde::{Deserialize, Serialize};
use std::{
    collections, env, fs,
    io::{self, Write},
    process::exit,
};

use crate::prime_math;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrimeTable {
    pub stored_primes: Vec<u32>,
}

impl PrimeTable {
    const fn new() -> Self {
        return Self {
            stored_primes: Vec::new(),
        };
    }

    #[must_use]
    pub fn from_vec(vec: Vec<u32>) -> Self {
        return Self {
            stored_primes: Vec::from_iter(vec),
        };
    }

    pub fn save(&mut self) {
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

    pub fn generate_primes(&mut self, num: u32) {
        let prev_primes = self.stored_primes.clone();
        if prev_primes.is_empty() {
            (1..=num)
                .filter(|uint| prime_math::is_prime(*uint))
                .for_each(|new_prime| self.stored_primes.push(new_prime));
            return;
        }
        (1..=num)
            .filter(|uint| prime_math::is_prime(*uint))
            .filter(|prime| !prev_primes.contains(prime))
            .for_each(|new_prime| self.stored_primes.push(new_prime));
    }

    pub fn get_primes_in_range(&mut self, input: u32) -> Vec<u32> {
        let output_vec = self
            .stored_primes
            .iter()
            .filter(|prime| **prime <= input)
            .copied()
            .collect::<Vec<u32>>();
        return output_vec;
    }
}
