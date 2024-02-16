// use xenakis_sieve::util::lcm;
use xenakis_sieve::Residual;


#[test]
fn test_residual_a() {
    let r1 = Residual::from_components(3, 0, false);
    assert_eq!(r1.to_string(), String::from("3@0"));
}

#[test]
#[should_panic]
fn test_residual_b() {
    let _r1 = Residual::from_components(0, 2, false);
}


#[test]
fn test_residual_from_repr_a() {
    let r1 = Residual::from_repr("3@1").expect("");
    assert_eq!(r1.to_string(), "3@1");
}

#[test]
fn test_residual_from_repr_b() {
    let r1 = Residual::from_repr("-3@4").expect("");
    assert_eq!(r1.to_string(), "-3@1");
}

#[test]
fn test_residual_from_repr_c() {
    let r1 = Residual::from_repr("-9@2").expect("");
    assert_eq!(r1.to_string(), "-9@2");
}


#[test]
fn test_residual_from_repr_d() {
    let r1 = Residual::from_repr("5@5").expect("");
    assert_eq!(r1.to_string(), "5@0");
}


#[test]
fn test_residual_to_string_a() {
    let r1 = Residual::from_components(3, 0, true);
    assert_eq!(r1.to_string(), String::from("-3@0"));
}

#[test]
fn test_residual_to_string_b() {
    let r1 = Residual::from_components(8, 3, true);
    assert_eq!(r1.to_string(), String::from("-8@3"));
}

#[test]
fn test_residual_to_string_c() {
    let r1 = Residual::from_components(5, 8, false);
    assert_eq!(r1.to_string(), String::from("5@3"));
}

#[test]
fn test_residual_to_string_d() {
    let r1 = Residual::from_components(5, 9, false);
    assert_eq!(r1.to_string(), String::from("5@4"));
}

#[test]
fn test_residual_to_string_e() {
    let r1 = Residual::from_components(5, 10, true);
    assert_eq!(r1.to_string(), String::from("-5@0"));
}

