use crate::prime_table::PrimeTable;

#[must_use]
fn floored_sqrt(x: u64) -> u64 {
    if x == 0 || x == 1 {
        return x;
    }

    let mut guess = x >> 1;

    while guess > x / guess {
        guess = (guess + x / guess) >> 1;
    }

    return guess;
}

fn is_stored_prime(num: u64, primes: &Vec<u64>) -> bool {
    if primes.is_empty() {
        return false;
    }

    if num > primes[primes.len() - 1] {
        return false;
    }

    return true;
}

#[must_use]
pub fn is_prime(num: u64, primes: &Vec<u64>) -> bool {
    if num == 1 || num == 0 {
        return false;
    }

    let upper = floored_sqrt(num);
    if num > 1 && upper == 1 {
        return true;
    }

    if is_stored_prime(num, primes) {
        return true;
    }

    let iter = match primes.is_empty() {
        true => (2..=upper).collect::<Vec<u64>>(),
        false => primes.clone().to_owned(),
    };

    #[allow(clippy::cast_precision_loss, clippy::cast_sign_loss)]
    for i in iter {
        if i > upper {
            break;
        }
        if num % i == 0 {
            return false;
        }
    }

    return true;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn sqrt_n() {
        let mut floored_sqrts: Vec<u64> = Vec::new();

        #[allow(clippy::cast_precision_loss, clippy::cast_sign_loss)]
        (0..1000u64).for_each(|i| floored_sqrts.push((i as f32).sqrt().floor() as u64));

        (0..floored_sqrts.len()).for_each(|i| {
            assert_eq!(floored_sqrt(i as u64), floored_sqrts[i]);
        });
    }
}
