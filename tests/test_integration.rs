// use xenakis_sieve::util::lcm;
use xenakis_sieve::Residual;


#[test]
fn test_residual_a() {
    let r1 = Residual::from_uint(3, 0);
    assert_eq!(r1.to_string(), String::from("3@0"));

}

#[test]
#[should_panic]
fn test_residual_b() {
    let _r1 = Residual::from_uint(0, 2);
}