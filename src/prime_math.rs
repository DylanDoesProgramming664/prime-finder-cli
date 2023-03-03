#[must_use]
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

#[must_use]
pub fn is_prime(num: u32) -> bool {
    if let 0..=1 = num {
        return false;
    }

    let upper = floored_sqrt(num);
    if num > 1 && upper == 1 {
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
        let mut floored_sqrts: Vec<u32> = Vec::new();

        #[allow(clippy::cast_precision_loss, clippy::cast_sign_loss)]
        (0..1000u32).for_each(|i| floored_sqrts.push((i as f32).sqrt().floor() as u32));

        (0..floored_sqrts.len()).for_each(|i| {
            assert_eq!(floored_sqrt(i as u32), floored_sqrts[i]);
        });
    }
}
