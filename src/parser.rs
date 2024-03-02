
use std::collections::VecDeque;


pub(crate) fn residual_to_ints(value: &str) -> Result<(u64, u64), &'static str> {
    let parts: Vec<&str> = value.split('@').collect();
    if parts.len() != 2 {
        return Err("Input must contain one '@' character separating two numbers.");
    }
    let m = parts[0].parse::<u64>().expect("Parse failure");
    let s = parts[1].parse::<u64>().expect("Parse failure");
    Ok((m, s))
}


#[inline(always)]
fn char_to_precedence(op: char) -> i8 {
    match op {
        '!' => 4,
        '&' => 3,
        '^' => 2,
        '|' => 1,
        _ => 0,
    }
}

#[inline(always)]
fn collect_operand(
    post: &mut VecDeque<String>,
    operand: &mut String,
) {
    if !operand.is_empty() {
        post.push_back(operand.clone());
        operand.clear();
    }
}

pub(crate) fn infix_to_postfix(expr: &str) -> Result<VecDeque<String>, &'static str> {
    let mut post: VecDeque<String> = VecDeque::new();
    let mut operators: Vec<char> = Vec::new();
    let mut operand: String = String::new();

    for c in expr.chars() {
        match c {
            '0'..='9' | '@' => operand.push(c), // operand characters
            '!' => operators.push(c),
            '|' | '&' | '^' => { // all binary operators
                collect_operand(&mut post, &mut operand);
                while let Some(&top) = operators.last() {
                    if top == '(' || char_to_precedence(top) < char_to_precedence(c) {
                        break;
                    }
                    post.push_back(operators.pop().unwrap().to_string())
                }
                operators.push(c);
            }
            '(' => operators.push(c),
            ')' => {
                collect_operand(&mut post, &mut operand);
                while let Some(top) = operators.pop() {
                    if top == '(' {
                        break;
                    }
                    post.push_back(top.to_string())
                }
            },
            _ if c.is_whitespace() => {},
            _ => {
                return Err("Found unsupported operator.");
            }
        }
    }
    // get any remaining numbers
    if !operand.is_empty() {
        post.push_back(operand);
    }
    // get any remaining operators
    while let Some(op) = operators.pop() {
        post.push_back(op.to_string());
    }
    Ok(post)
}


#[cfg(test)] // only compile when running cargo test
mod tests {
    use super::*; // bring code in outer into scope
    // use crate::util::*;

    // {default} % cargo test test_infix_to_rpn_a -- --nocapture
    //------------------------------------------------------------------------------
    #[test]
    fn test_infix_to_postfix_a() {
        let e1 = "!3@1 & 6@2 | !(10@0 | 2@0 | 3@0 )";
        let px1 = infix_to_postfix(e1).unwrap();
        assert_eq!(
            px1.iter().collect::<Vec<_>>(),
            vec!["3@1", "!", "6@2", "&", "10@0", "2@0", "|", "3@0", "|", "!", "|"]
            );
    }

    #[test]
    fn test_infix_to_postfix_b() {
        let e1 = "10@0 | 2@0 | 3@0";
        let px1 = infix_to_postfix(e1).unwrap();
        assert_eq!(
            px1.iter().collect::<Vec<_>>(),
            vec!["10@0", "2@0", "|", "3@0", "|"]
            );
    }

    #[test]
    fn test_infix_to_postfix_c() {
        let e1 = "!10@0 | !2@0 | !3@0";
        let px1 = infix_to_postfix(e1).unwrap();
        assert_eq!(
            px1.iter().collect::<Vec<_>>(),
            vec!["10@0", "!", "2@0", "!", "|", "3@0", "!", "|"]
            );
    }

    #[test]
    fn test_infix_to_postfix_d() {
        let e1 = "(10@0 & !2@0) | (!3@0 & 4@2)";
        let px1 = infix_to_postfix(e1).unwrap();
        assert_eq!(
            px1.iter().collect::<Vec<_>>(),
            vec!["10@0", "2@0", "!", "&", "3@0", "!", "4@2", "&", "|"]
            );
    }

    #[test]
    fn test_infix_to_postfix_e() {
        let e1 = "10@0 ^ 10@9";
        let px1 = infix_to_postfix(e1).unwrap();
        assert_eq!(
            px1.iter().collect::<Vec<_>>(),
            vec!["10@0", "10@9", "^"]
            );
    }

}

