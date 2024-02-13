use xenakis_sieve::util::lcm;
use xenakis_sieve::util::gcd;


#[test]
fn test_lcm_gcd_a() {
    assert_eq!(3 * 4 / gcd(3, 4), lcm(3, 4));
}