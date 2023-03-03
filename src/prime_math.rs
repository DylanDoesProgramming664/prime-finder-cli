use crate::prime_table::PrimeTable;

#[must_use]
const fn floored_sqrt(x: u64) -> u64 {
    if let 0..=1 = x {
        return x;
    }

    let mut guess = x >> 1;

    while guess * guess > x {
        guess = (guess + x / guess) >> 1;
    }

    return guess;
}

fn is_stored_prime(num: u64, primes: &Vec<u64>) -> bool {
    if num > primes[primes.len() - 1] {
        return false;
    }

    let mut upper = primes.len() - 1;
    let mut lower: usize = 0;
    while lower <= upper {
        let mid = (upper + lower) >> 1;
        if primes[mid] < num {
            lower = mid + 1;
            continue;
        } else if primes[mid] > num {
            upper = mid - 1;
        } else {
            return true;
        }
    }

    return false;
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

    #[allow(clippy::cast_precision_loss, clippy::cast_sign_loss)]
    for i in 2..=upper {
        if (num as f32) % (i as f32) == 0f32 {
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
