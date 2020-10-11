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
    let mut max = 0;
    let mut current = 0;
    for c in s.chars() {
        match c {
            '(' => current += 1,
            ')' => current -= 1,
            _ => (),
        }
        if current > max {
            max = current;
        }
    }
    max
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
