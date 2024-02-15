use std::fmt;


mod util {

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

    // This is a brute-force implementation of modular inverse. The Extended Euclidian Algorithm might be a better choice.
    fn meziriac(a: u64, b: u64) -> u64 {
        let mut g: u64 = 1;
        if b == 1 {
            g = 1;
        } else if a == b {
            g = 0;
        } else {
            while g < u64::MAX {
                if ((g * a) % b) == 1 {
                    break
                }
                g += 1;
            }
        }
        g
    }

    #[cfg(test)] // only compile when running cargo test
    mod tests {
        use super::*; // bring code in outer into scope
        // use crate::util::*;

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

        #[test]
        fn test_meziriac_a() {
            assert_eq!(meziriac(1, 1), 1);
            assert_eq!(meziriac(10, 1), 1);
            assert_eq!(meziriac(10, 10), 0);
            assert_eq!(meziriac(12, 12), 0);
            assert_eq!(meziriac(3, 11), 4);
            assert_eq!(meziriac(20, 9), 5);
            assert_eq!(meziriac(101, 13), 4);
        }

    }

}


pub struct Residual {
    modulus: u64,
    shift: u64,
    invert: bool,
}


impl Residual {

    pub fn from_components(modulus: u64, shift: u64, invert: bool) -> Self {
        assert!(modulus > 0);
        let shift = shift % modulus;
        Self{modulus: modulus, shift: shift, invert: invert}
    }

    pub fn from_repr(value: &str) -> Result<Self, String> {
        let parts: Vec<&str> = value.split('@').collect();
        if parts.len() != 2 {
            return Err("Input must contain one '@' character separating two numbers.".to_string());
        }
        let m = parts[0].parse::<u64>().map_err(|_| "Parse failure.".to_string())?;
        let s = parts[1].parse::<u64>().map_err(|_| "Parse failure.".to_string())?;
        Ok(Self::from_components(m, s, false))
    }

}

impl fmt::Display for Residual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let n = if self.invert {String::from("-")} else {String::new()};
        write!(f, "{}{}@{}", n, self.modulus, self.shift)
    }
}