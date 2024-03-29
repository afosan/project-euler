//! https://projecteuler.net/problem=2

fn solve(n: u64) -> u64 {
    let mut f0 = 1;
    let mut f1 = 1;
    let mut s = 0;

    while f1 <= n {
        if f1 % 2 == 0 {
            s += f1;
        }
        (f0, f1) = (f1, f0 + f1);
    }

    s
}

fn main() {
    let answer = solve(4_000_000);
    println!("{answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = solve(100);
        assert_eq!(result, 44);
    }
}
