use std::fmt;
use std::ops::Not;
use std::ops::BitAnd;
use std::ops::BitOr;
// use std::ops::Range;
use std::cmp::Ordering;
// use std::ops::RangeBounds;


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

    // fn lcm(a: u64, b: u64) -> u64 {
    //     a * b / gcd(a, b)
    // }

    /// This is a brute-force implementation of modular inverse. The Extended Euclidian Algorithm might be a better choice.
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

    /// Intersection of two residual classes.
    pub fn intersection(
            m1: u64,
            m2: u64,
            mut s1: u64,
            mut s2: u64,
            ) -> (u64, u64) {
        if m1 == 0 || m2 == 0 {
            // intersection of null and anything is null
            return (0, 0);
        }
        // normalize shifts
        s1 = s1 % m1;
        s2 = s2 % m2;

        // use common divisor
        let d = gcd(m1, m2);
        let md1 = m1 / d;
        let md2 = m2 / d;
        let span: u64 = (s2 as i128 - s1 as i128).abs().try_into().unwrap();

        if d != 1 && (span % d != 0) {
            return (0, 0); // no intersection
        }
        if d != 1
            && (span % d == 0)
            && (s1 != s2)
            && (md1 == md2) {
            return (d, s1);
        }
        // d might be 1
        let m = md1 * md2 * d;
        (m, (s1 + (meziriac(md1, md2) * span * md1)) % m)

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

        // #[test]
        // fn test_lcm_a() {
        //     assert_eq!(lcm(12, 8), 24);
        // }

        // #[test]
        // fn test_lcm_b() {
        //     assert_eq!(lcm(3, 4), 12);
        // }

        // #[test]
        // #[should_panic]
        // fn test_lcm_c() {
        //     // as gcd panics on 0, this does as well
        //     assert_eq!(lcm(3, 0), 0);
        // }

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

//------------------------------------------------------------------------------

/// Container of integer values for the modulus and the shift of a Residual class.
///
/// # Fields
/// * `modulus` - The modulus.
/// * `shift` - The shift.
///
#[derive(Clone, Debug)]
pub struct Residual {
    modulus: u64,
    shift: u64,
}

impl Residual {

    pub fn from_components(modulus: u64, mut shift: u64) -> Self {
        if modulus == 0 {
            shift = 0;
        } else {
            shift %= modulus;
        }
        Self{modulus: modulus, shift: shift}
    }

    pub fn from_repr(value: &str) -> Result<Self, String> {
        // let invert;
        // if value.starts_with('!') {
        //     invert = true;
        //     value = &value[1..];
        // } else {
        //     invert = false;
        // }

        let parts: Vec<&str> = value.split('@').collect();
        if parts.len() != 2 {
            return Err("Input must contain one '@' character separating two numbers.".to_string());
        }
        let m = parts[0].parse::<u64>().map_err(|_| "Parse failure.".to_string())?;
        let s = parts[1].parse::<u64>().map_err(|_| "Parse failure.".to_string())?;
        Ok(Self::from_components(m, s))
    }

    /// Return `true` if the values is contained with this Sieve.
    ///
    pub fn isin(&self, value: i128) -> bool {
        if self.modulus == 0 {
            return false;
        }
        let pos: i128 = value - self.shift as i128;
        pos % self.modulus as i128 == 0
    }

}

impl fmt::Display for Residual {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let n = if self.invert {String::from("!")} else {String::new()};
        write!(f, "{}@{}", self.modulus, self.shift)
    }
}

// impl Not for Residual {
//     type Output = Self;

//     fn not(self) -> Self {
//         Self::from_components(self.modulus, self.shift, !self.invert)
//     }
// }

impl BitAnd for Residual {
    type Output = Residual;

    fn bitand(self, rhs: Self) -> Self::Output {
        let (m, s) = util::intersection(
                self.modulus,
                rhs.modulus,
                self.shift,
                rhs.shift,
                );
        Self::from_components(m, s)
        }
}

impl PartialEq for Residual {
    fn eq(&self, other: &Self) -> bool {
        self.modulus == other.modulus && self.shift == other.shift
    }
}

impl Eq for Residual {}

impl PartialOrd for Residual {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Residual {
    fn cmp(&self, other: &Self) -> Ordering {
        self.modulus.cmp(&other.modulus)
            .then_with(|| self.shift.cmp(&other.shift))
    }
}

//------------------------------------------------------------------------------

/// A node in the graph of Residuals combined by logical operations.
///
#[derive(Clone, Debug)]
pub enum SieveNode {
    Unit(Residual),
    Intersection(Box<SieveNode>, Box<SieveNode>),
    Union(Box<SieveNode>, Box<SieveNode>),
    Inversion(Box<SieveNode>),
}

impl fmt::Display for SieveNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String;
        match self {
            SieveNode::Unit(residual) => {
                s = residual.to_string();
            },
            SieveNode::Intersection(lhs, rhs) => {
                let lhs_str = lhs.to_string();
                let rhs_str = rhs.to_string();
                s = format!("{lhs_str}&{rhs_str}");
            },
            SieveNode::Union(lhs, rhs) => {
                let lhs_str = lhs.to_string();
                let rhs_str = rhs.to_string();
                s = format!("{lhs_str}|{rhs_str}");
            },
            SieveNode::Inversion(part) => {
                let r = part.to_string();
                s = format!("!({r})");
            },
        }
        write!(f, "{}", s)
    }
}

impl SieveNode
{
    /// Return `true` if the values is contained with this Sieve.
    ///
    pub fn isin(&self, value: i128) -> bool {
        match self {
            SieveNode::Unit(residual) => {
                residual.isin(value)
            },
            SieveNode::Intersection(lhs, rhs) => {
                lhs.isin(value) && rhs.isin(value)
            },
            SieveNode::Union(lhs, rhs) => {
                lhs.isin(value) || rhs.isin(value)
            },
            SieveNode::Inversion(part) => {
                !part.isin(value)
            },
        }
    }
}

//------------------------------------------------------------------------------

/// A Sieve.
///
#[derive(Clone, Debug)]
pub struct Sieve {
    root: SieveNode, // should this be boxed?
}


impl BitAnd for Sieve {
    type Output = Sieve;

    fn bitand(self, rhs: Self) -> Self::Output {
        Sieve{root: SieveNode::Intersection(Box::new(self.root), Box::new(rhs.root))}
    }
}

impl BitOr for Sieve {
    type Output = Sieve;

    fn bitor(self, rhs: Self) -> Self::Output {
        Sieve{root: SieveNode::Union(Box::new(self.root), Box::new(rhs.root))}
    }
}

impl Not for Sieve {
    type Output = Sieve;

    fn not(self) -> Self::Output {
        Sieve{root: SieveNode::Inversion(Box::new(self.root))}
    }
}

impl fmt::Display for Sieve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sieve{{{}}}", self.root.to_string())
    }
}

impl Sieve {
    /// Construct a Sieve from a Residual string representation.
    ///
    pub fn r(value: &str) -> Self {
        match Residual::from_repr(value) {
            Ok(residual) => Self{root: SieveNode::Unit(residual)},
            Err(error) => panic!("Could not create Residual: {:?}", error),
        }

    }
    /// Return `true` if the values is contained with this Sieve.
    ///
    pub fn isin(&self, value: i128) -> bool {
        self.root.isin(value)
    }

    /// Iterate over values contained within the sieve.
    pub fn iter_value(&self, iterator: impl Iterator<Item = i128>) -> SieveIterateValue<impl Iterator<Item = i128>> {
        // NOTE: do not want to clone self here...
        // assert!(end >= start);
        SieveIterateValue{iterator: iterator, sieve_node: self.root.clone()}
    }
}

//------------------------------------------------------------------------------

pub struct SieveIterateValue<I>
where
    I: Iterator<Item = i128>
{
    iterator: I,
    sieve_node: SieveNode,
}

impl<I> Iterator for SieveIterateValue<I>
where
    I: Iterator<Item = i128>
{
    type Item = i128;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(p) = self.iterator.next() {
            if self.sieve_node.isin(p) {
                return Some(p);
            }
        }
        None
    }
}