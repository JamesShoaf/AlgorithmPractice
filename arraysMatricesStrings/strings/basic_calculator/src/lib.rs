/*
Implement a basic calculator to evaluate a simple expression string.

The expression string contains only non-negative integers, +, -, *, / operators and empty spaces.
The integer division should truncate toward zero.
*/

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
    End,
}

struct Calc {
    val: i32,
    op: Operation,
}

impl Calc {
    fn new(val: i32) -> Self {
        Calc {
            val,
            op: Operation::End,
        }
    }
}

fn update_op(current: &mut Option<Calc>, op: Operation, vec: &mut Vec<Calc>) -> Result<(), ()> {
    if let Some(mut calc) = current.take() {
        calc.op = op;
        vec.push(calc);
        Ok(())
    } else {
        Err(())
    }
}

fn parse(s: String) -> Result<Vec<Calc>, ()> {
    let mut res: Vec<Calc> = Vec::new();
    let mut current: Option<Calc> = None;
    for c in s.chars() {
        match c {
            '+' => update_op(&mut current, Operation::Add, &mut res)?,
            '-' => update_op(&mut current, Operation::Subtract, &mut res)?,
            '*' => update_op(&mut current, Operation::Multiply, &mut res)?,
            '/' => update_op(&mut current, Operation::Divide, &mut res)?,
            _ => {
                if let Some(digit) = c.to_digit(10) {
                    let digit = digit as i32;
                    if let Some(calc) = current.as_mut() {
                        calc.val *= 10;
                        calc.val += digit;
                    } else {
                        current = Some(Calc::new(digit));
                    }
                }
            }
        }
    }
    if let Some(calc) = current {
        res.push(calc);
        Ok(res)
    } else {
        Err(())
    }
}

fn multiply_and_divide(vec: Vec<Calc>) -> Vec<Calc> {
    let mut res = Vec::new();
    let mut current: Option<Calc> = None;
    for calc in vec {
        if let Some(mut curr) = current.take() {
            match curr.op {
                Operation::Multiply => {
                    curr.val *= calc.val;
                    curr.op = calc.op;
                    current.replace(curr);
                }
                Operation::Divide => {
                    curr.val /= calc.val;
                    curr.op = calc.op;
                    current.replace(curr);
                }
                _ => {
                    res.push(curr);
                    current.replace(calc);
                }
            }
        } else {
            current = Some(calc);
        }
    }
    if let Some(calc) = current {
        res.push(calc);
    }
    res
}

fn add_and_subtract(vec: Vec<Calc>) -> Option<Calc> {
    let mut current: Option<Calc> = None;
    for calc in vec {
        if let Some(curr) = current.as_mut() {
            match curr.op {
                Operation::Add => {
                    curr.val += calc.val;
                    curr.op = calc.op;
                }
                Operation::Subtract => {
                    curr.val -= calc.val;
                    curr.op = calc.op;
                }
                _ => (),
            }
        } else {
            current = Some(calc);
        }
    }
    current
}

pub fn calculate(s: String) -> i32 {
    let parse = parse(s).unwrap();
    let mult_and_div = multiply_and_divide(parse);
    if let Some(calc) = add_and_subtract(mult_and_div) {
        calc.val
    } else {
        0
    }
}

/*
#1 solution for comparision
pub fn calculate(s: String) -> i32 {
    let bytes = s.as_bytes();
    let (mut prev, mut cur) = (0, 0);
    let mut res = 0;
    let mut sign = b'+';
    for byte in bytes {
        let is_digit = (byte as char).is_digit(10);
        if is_digit {
            cur = cur * 10 + (byte - b'0') as i32;
        }
        if (!is_digit && byte != b' ') || i + 1 == bytes.len() {
            match sign {
                b'+' => {
                    res += prev;
                    prev = cur;
                }
                b'-' => {
                    res += prev;
                    prev = -cur;
                }
                b'*' => prev *= cur,
                b'/' => prev /= cur,
                _ => (),
            }
            cur = 0;
            sign = byte;
        }
    }
    res += prev;
    res
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("1"), 1),
            (String::from(" 1"), 1),
            (String::from("1 "), 1),
            (String::from("2/1"), 2),
            (String::from("1/2"), 0),
            (String::from("3*2"), 6),
            (String::from("1/3*2"), 0),
            (String::from("8/3*2"), 4),
            (String::from("3+5 / 2 "), 5),
            (String::from("3-5/2"), 1),
            (String::from("5/2-4"), -2),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(calculate(s), expected);
        }
    }
    #[test]
    #[should_panic]
    fn panics_on_lone_plus() {
        calculate(String::from("+"));
    }
    #[test]
    #[should_panic]
    fn panics_on_terminal_plus() {
        calculate(String::from("1 +"));
    }
    #[test]
    #[should_panic]
    fn panics_on_lone_minus() {
        calculate(String::from("-"));
    }
    #[test]
    #[should_panic]
    fn panics_on_terminal_minus() {
        calculate(String::from("1 -"));
    }
    #[test]
    #[should_panic]
    fn panics_on_lone_multiply() {
        calculate(String::from("*"));
    }
    #[test]
    #[should_panic]
    fn panics_on_terminal_multiply() {
        calculate(String::from("1 *"));
    }
    #[test]
    #[should_panic]
    fn panics_on_lone_divide() {
        calculate(String::from("/"));
    }
    #[test]
    #[should_panic]
    fn panics_on_terminal_divide() {
        calculate(String::from("1 /"));
    }
    #[test]
    #[should_panic]
    fn panics_on_divide_by_zero() {
        calculate(String::from("1/0"));
    }
}
