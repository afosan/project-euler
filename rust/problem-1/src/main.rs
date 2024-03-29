//! https://projecteuler.net/problem=1

fn solve(n: u64) -> u64 {
    (1..n).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}

fn main() {
    let answer = solve(1000);
    println!("{answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve(10);
        assert_eq!(result, 23);
    }
}
