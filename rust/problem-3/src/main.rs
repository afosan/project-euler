//! https://projecteuler.net/problem=3

fn largest_prime_factor(n: u64) -> u64 {
    let t = (n as f64).sqrt() as u64;
    let mut ps = vec![];

    (2..=t).for_each(|i| {
        if ps.iter().all(|p| i % p != 0) {
            ps.push(i);
        }
    });

    *ps.iter().rev().filter(|p| n % **p == 0).nth(0).unwrap()
}

fn main() {
    let answer = largest_prime_factor(600_851_475_143);
    println!("{answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = largest_prime_factor(13195);
        assert_eq!(result, 29);
    }
}
