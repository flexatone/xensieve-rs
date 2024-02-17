// use xenakis_sieve::util::lcm;
use xenakis_sieve::Residual;


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

//------------------------------------------------------------------------------

#[test]
fn test_residual_at_a() {
    let r1 = Residual::from_components(3, 0);
    assert_eq!(r1.at(-3), true);
    assert_eq!(r1.at(-2), false);
    assert_eq!(r1.at(-1), false);
    assert_eq!(r1.at(0), true);
    assert_eq!(r1.at(1), false);
    assert_eq!(r1.at(2), false);
    assert_eq!(r1.at(3), true);
    assert_eq!(r1.at(4), false);
    assert_eq!(r1.at(5), false);

}

#[test]
fn test_residual_at_b() {
    let r1 = Residual::from_components(0, 0);
    assert_eq!(r1.at(-2), false);
    assert_eq!(r1.at(-1), false);
    assert_eq!(r1.at(0), false);
    assert_eq!(r1.at(1), false);
    assert_eq!(r1.at(2), false);
    assert_eq!(r1.at(3), false);
}
