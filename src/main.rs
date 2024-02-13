

fn gcd(mut n: u64, mut m: u64) -> u64 {
    // not sure if assert is best way to handle this
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}



fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*; // bring code in outer into scope

    #[test]
    fn test_gcd_a() {
        assert_eq!(gcd(14, 15), 1);
    }

    #[test]
    fn test_gcd_b() {
        assert_eq!(gcd(12, 8), 4);
    }

    #[test]
    fn test_gcd_c() {
        let a = 2 * 3 * 5 * 11 * 17;
        let b = 3 * 7 * 11 * 13 * 19;
        assert_eq!(gcd(a, b), 3 * 11);
    }

    #[test]
    #[should_panic]
    fn test_gcd_d() {
        gcd(12, 0);
    }

    #[test]
    #[should_panic]
    fn test_gcd_e() {
        gcd(0, 3);
    }

    #[test]
    fn test_lcm_a() {
        assert_eq!(lcm(12, 8), 24);
    }

    #[test]
    fn test_lcm_b() {
        assert_eq!(lcm(3, 4), 12);
    }

    #[test]
    #[should_panic]
    fn test_lcm_c() {
        // as gcd panics on 0, this does as well
        assert_eq!(lcm(3, 0), 0);
    }

}



