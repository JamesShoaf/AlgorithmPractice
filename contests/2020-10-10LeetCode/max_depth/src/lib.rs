/* 
A string is a valid parentheses string (denoted VPS) if it meets one of the following:

    It is an empty string "", or a single character not equal to "(" or ")",
    It can be written as AB (A concatenated with B), where A and B are VPS's, or
    It can be written as (A), where A is a VPS.

We can similarly define the nesting depth depth(S) of any VPS S as follows:

    depth("") = 0
    depth(A + B) = max(depth(A), depth(B)), where A and B are VPS's
    depth("(" + A + ")") = 1 + depth(A), where A is a VPS.

For example, "", "()()", and "()(()())" are VPS's (with nesting depths 0, 1, and 2), and ")(" and "(()" are not VPS's.

Given a VPS represented as string s, return the nesting depth of s.
*/

pub fn max_depth(s:String) -> i32 {
    s.chars().fold((0, 0), |(max, curr), x| match x {
        '(' => (if curr == max { max + 1 } else { max }, curr + 1),
        ')' => (max, curr - 1),
        _ => (max, curr)}
    ).0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from(""), 0),
            (String::from("(1+(2*3)+((8))/4))+1"), 3),
        ];
        for (s, expected) in test_tuples {
            assert_eq!(max_depth(s), expected);
        }
    }
}
