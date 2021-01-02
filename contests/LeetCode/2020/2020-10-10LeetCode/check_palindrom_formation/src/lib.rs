/* 
You are given two strings a and b of the same length. Choose an index and split both strings at the
same index, splitting a into two strings: aprefix and asuffix where a = aprefix + asuffix, and
splitting b into two strings: bprefix and bsuffix where b = bprefix + bsuffix.
Check if aprefix + bsuffix or bprefix + asuffix forms a palindrome.

When you split a string s into sprefix and ssuffix, either ssuffix or sprefix is allowed to be empty.
For example, if s = "abc", then "" + "abc", "a" + "bc", "ab" + "c" , and "abc" + "" are valid splits.

Return true if it is possible to form a palindrome string, otherwise return false.

Notice that x + y denotes the concatenation of strings x and y.
*/

// fn is_palindrome(s: &String) -> bool {
//     for (c, d) in s.chars().zip(s.chars().rev()) {
//         if c != d { return false; }
//     }
//     true
// }

fn palindrome_at_split(split: usize, a: &Vec<char>, b: &Vec<char>) -> bool {
    for i in 0..=a.len() / 2 {
        let pre = if i < split { a[i] } else { b[i] };
        let post = if a.len() - i - 1 < split { a[a.len() - i - 1] } else { b[a.len() - i - 1] };
        if pre != post { 
            return false;
        }
    }
    true
}

fn check_palindrome_formation(a: String, b:String) -> bool {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    for i in 0..=a.len() {
        if palindrome_at_split(i, &a_chars, &b_chars) || palindrome_at_split(i, &b_chars, &a_chars) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*; 
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("x"), String::from("y"), true),
            (String::from("abdef"), String::from("fgcab"), false),
            (String::from("abdef"), String::from("fecab"), true),
            (String::from("ulacfd"), String::from("jizalu"), true),
        ];
        for (a, b, expected) in test_tuples {
            assert_eq!(check_palindrome_formation(a, b), expected);
        }
    }
}
