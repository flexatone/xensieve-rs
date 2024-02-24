use std::fmt;
use std::ops::Not;
use std::ops::BitAnd;
use std::ops::BitOr;
use std::cmp::Ordering;
mod util;

//------------------------------------------------------------------------------

/// Container of integer values for the modulus and the shift of a Residual class.
///
/// # Fields
/// * `modulus` - The modulus.
/// * `shift` - The shift.
///
#[derive(Clone, Debug)]
pub(crate) struct Residual {
    modulus: u64,
    shift: u64,
}

impl Residual {

    pub(crate) fn from_components(modulus: u64, mut shift: u64) -> Self {
        if modulus == 0 {
            shift = 0;
        } else {
            shift %= modulus;
        }
        Self{modulus: modulus, shift: shift}
    }

    pub(crate) fn from_repr(value: &str) -> Result<Self, String> {
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
    pub(crate) fn isin(&self, value: i128) -> bool {
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
pub(crate) enum SieveNode {
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
    /// NOTE: the iterator "impl" type shows that the parameter must implement the Iterator<Item = i128> trait.
    pub fn iter_value(&self, iterator: impl Iterator<Item = i128>) -> SieveIterateValue<impl Iterator<Item = i128>> {
        // NOTE: do not want to clone self here...
        SieveIterateValue{iterator: iterator, sieve_node: self.root.clone()}
    }

    /// Iterate over Boolean states contained within the sieve.
    pub fn iter_state(&self, iterator: impl Iterator<Item = i128>) -> SieveIterateState<impl Iterator<Item = i128>> {
        SieveIterateState{iterator: iterator, sieve_node: self.root.clone()}
    }

    /// Iterate over integer intervals between values in the sieve.
    pub fn iter_interval(&self, iterator: impl Iterator<Item = i128>) -> SieveIterateInterval<impl Iterator<Item = i128>> {
        SieveIterateInterval{iterator: iterator, sieve_node: self.root.clone(), last: PositionLast::Init}
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

//------------------------------------------------------------------------------

pub struct SieveIterateState<I>
where
    I: Iterator<Item = i128>
{
    iterator: I,
    sieve_node: SieveNode,
}

impl<I> Iterator for SieveIterateState<I>
where
    I: Iterator<Item = i128>
{
    type Item = bool;

    fn next(&mut self) -> Option<Self::Item> {
        match self.iterator.next() {
            Some(p) => Some(self.sieve_node.isin(p)),
            None => None,
        }
    }
}

//------------------------------------------------------------------------------

enum PositionLast {
    Init,
    Value(i128),
}

pub struct SieveIterateInterval<I>
where
    I: Iterator<Item = i128>
{
    iterator: I,
    sieve_node: SieveNode,
    last: PositionLast,
}

impl<I> Iterator for SieveIterateInterval<I>
where
    I: Iterator<Item = i128>
{
    type Item = i128;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(p) = self.iterator.next() {
            if self.sieve_node.isin(p) {
                match self.last {
                    PositionLast::Init => {
                        // drop the first value
                        self.last = PositionLast::Value(p);
                        continue;
                    },
                    PositionLast::Value(last) => {
                        let post = p - last;
                        self.last = PositionLast::Value(p);
                        return Some(post);
                    }
                }
            }
        }
        None
    }
}

//------------------------------------------------------------------------------

#[cfg(test)] // only compile when running cargo test
mod tests {
    use super::*; // bring code in outer into scope
    // use crate::util::*;

    #[test]
    fn test_residual_a() {
        let r1 = Residual::from_components(3, 0);
        assert_eq!(r1.to_string(), String::from("3@0"));
    }

    #[test]
    fn test_residual_b() {
        let r1 = Residual::from_components(0, 2);
        assert_eq!(r1.to_string(), "0@0");
    }

    //------------------------------------------------------------------------------
    #[test]
    fn test_residual_from_repr_a() {
        let r1 = Residual::from_repr("3@1").expect("");
        assert_eq!(r1.to_string(), "3@1");
    }

    #[test]
    fn test_residual_from_repr_b() {
        let r1 = Residual::from_repr("3@4").expect("");
        assert_eq!(r1.to_string(), "3@1");
    }

    #[test]
    fn test_residual_from_repr_c() {
        let r1 = Residual::from_repr("9@2").expect("");
        assert_eq!(r1.to_string(), "9@2");
    }


    #[test]
    fn test_residual_from_repr_d() {
        let r1 = Residual::from_repr("5@5").expect("");
        assert_eq!(r1.to_string(), "5@0");
    }

    #[test]
    fn test_residual_from_repr_e() {
        let r1 = Residual::from_repr("0@5").expect("");
        assert_eq!(r1.to_string(), "0@0");
    }

    //------------------------------------------------------------------------------
    #[test]
    fn test_residual_to_string_a() {
        let r1 = Residual::from_components(3, 0);
        assert_eq!(r1.to_string(), "3@0");
    }

    #[test]
    fn test_residual_to_string_b() {
        let r1 = Residual::from_components(8, 3);
        assert_eq!(r1.to_string(), "8@3");
    }

    #[test]
    fn test_residual_to_string_c() {
        let r1 = Residual::from_components(5, 8);
        assert_eq!(r1.to_string(), "5@3");
    }

    #[test]
    fn test_residual_to_string_d() {
        let r1 = Residual::from_components(5, 9);
        assert_eq!(r1.to_string(), "5@4");
    }

    #[test]
    fn test_residual_to_string_e() {
        let r1 = Residual::from_components(5, 10);
        assert_eq!(r1.to_string(), "5@0");
    }

    //------------------------------------------------------------------------------

    // #[test]
    // fn test_residual_not_a() {
    //     let r1 = Residual::from_components(5, 10);
    //     assert_eq!(r1.to_string(), String::from("!5@0"));
    //     let r2 = !r1;
    //     assert_eq!(r2.to_string(), "5@0");
    //     let r3 = !r2;
    //     assert_eq!(r3.to_string(), "!5@0");
    // }

    #[test]
    fn test_residual_eq_a() {
        let r1 = Residual::from_components(5, 2);
        let r2 = Residual::from_components(5, 3);
        assert_eq!(r1 == r2, false);
        assert_eq!(r1 != r2, true);
    }

    #[test]
    fn test_residual_eq_b() {
        let r1 = Residual::from_components(5, 2);
        let r2 = Residual::from_components(5, 2);
        assert_eq!(r1 == r2, true);
        assert_eq!(r1 != r2, false);

    }

    #[test]
    fn test_residual_ord_a() {
        let r1 = Residual::from_components(5, 2);
        let r2 = Residual::from_components(5, 3);
        assert!(r1 < r2);
    }

    #[test]
    fn test_residual_ord_b() {
        let r1 = Residual::from_components(2, 3);
        let r2 = Residual::from_components(5, 3);
        assert!(r1 < r2);
    }

    #[test]
    fn test_residual_ord_c() {
        let r1 = Residual::from_components(5, 3);
        let r2 = Residual::from_components(5, 3);
        assert!(r1 == r2);
    }

    //------------------------------------------------------------------------------

    #[test]
    fn test_residual_bitand_a() {
        let r1 = Residual::from_components(4, 0);
        let r2 = Residual::from_components(3, 0);
        assert_eq!((r1 & r2).to_string(), "12@0");
    }

    #[test]
    fn test_residual_bitand_b() {
        let r1 = Residual::from_components(4, 0);
        let r2 = Residual::from_components(3, 1);
        assert_eq!((r1 & r2).to_string(), "12@4");
    }

    #[test]
    fn test_residual_bitand_c() {
        let r1 = Residual::from_components(5, 2);
        let r2 = Residual::from_components(10, 3);
        assert_eq!((r1 & r2).to_string(), "0@0");
    }

    #[test]
    fn test_residual_bitand_d() {
        let r1 = Residual::from_components(3, 2);
        let r2 = Residual::from_components(3, 1);
        assert_eq!((r1 & r2).to_string(), "0@0");
    }

    //--------------------------------------------------------------------------

    #[test]
    fn test_residual_isin_a() {
        let r1 = Residual::from_components(3, 0);
        assert_eq!(r1.isin(-3), true);
        assert_eq!(r1.isin(-2), false);
        assert_eq!(r1.isin(-1), false);
        assert_eq!(r1.isin(0), true);
        assert_eq!(r1.isin(1), false);
        assert_eq!(r1.isin(2), false);
        assert_eq!(r1.isin(3), true);
        assert_eq!(r1.isin(4), false);
        assert_eq!(r1.isin(5), false);

    }

    #[test]
    fn test_residual_isin_b() {
        let r1 = Residual::from_components(0, 0);
        assert_eq!(r1.isin(-2), false);
        assert_eq!(r1.isin(-1), false);
        assert_eq!(r1.isin(0), false);
        assert_eq!(r1.isin(1), false);
        assert_eq!(r1.isin(2), false);
        assert_eq!(r1.isin(3), false);
    }

    #[test]
    fn test_residual_isin_c() {
        let r1 = Residual::from_components(3, 1);
        assert_eq!(r1.isin(-3), false);
        assert_eq!(r1.isin(-2), true);
        assert_eq!(r1.isin(-1), false);
        assert_eq!(r1.isin(0), false);
        assert_eq!(r1.isin(1), true);
        assert_eq!(r1.isin(2), false);
        assert_eq!(r1.isin(3), false);
        assert_eq!(r1.isin(4), true);
    }

    //--------------------------------------------------------------------------

    #[test]
    fn test_sieve_isin_a() {
        let r1 = Residual::from_components(3, 0);
        let s1 = SieveNode::Unit(r1);

        let pos = vec![-3,   -2,    -1,    0,    1];
        let val = vec![true, false, false, true, false];
        for (p, b) in pos.iter().zip(val.iter()) {
            assert_eq!(s1.isin(*p), *b);
        }
    }

    #[test]
    fn test_sieve_isin_b() {
        let r1 = Residual::from_components(3, 0);
        let r2 = Residual::from_components(3, 1);
        let s1 = SieveNode::Union(
                Box::new(SieveNode::Unit(r1)),
                Box::new(SieveNode::Unit(r2)),
            );

        assert_eq!(s1.isin(-2), true);
        assert_eq!(s1.isin(-1), false);
        assert_eq!(s1.isin(0), true);
        assert_eq!(s1.isin(1), true);
        assert_eq!(s1.isin(2), false);
        assert_eq!(s1.isin(3), true);
        assert_eq!(s1.isin(4), true);
    }

}