/*
Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.

An input string is valid if:

    Open brackets must be closed by the same type of brackets.
    Open brackets must be closed in the correct order.

*/

fn map_bracket(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        _ => '😮',
    }
}

pub fn is_valid(s: String) -> bool {
    let mut stack: Vec<char> = Vec::new();
    for c in s.chars() {
        match c {
            ')' | ']' | '}' => {
                if stack.pop() != Some(map_bracket(c)) {
                    return false;
                }
            }
            _ => stack.push(c),
        }
    }
    stack.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            ("()".to_string(), true),
            ("()[]{}".to_string(), true),
            ("(]".to_string(), false),
            ("([)]".to_string(), false),
            ("{[]}".to_string(), true),
            ("".to_string(), true),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(is_valid(s), expected);
        }
    }
}
