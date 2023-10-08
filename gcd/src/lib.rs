pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    assert!(a != 0 || b != 0);
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(14, 15), 1);
        assert_eq!(gcd(2 * 3 * 5 * 11 * 17,
                       3 * 7 * 11 * 13 * 19),
                   3 * 11);
    }
}
