/*
Given a string s consisting only of characters 'a', 'b', and 'c'. You are asked to apply
the following algorithm on the string any number of times:

    Pick a non-empty prefix from the string s where all the characters in the prefix are equal.
    Pick a non-empty suffix from the string s where all the characters in this suffix are equal.
    The prefix and the suffix should not intersect at any index.
    The characters from the prefix and suffix must be the same.
    Delete both the prefix and the suffix.

Return the minimum length of s after performing the above operation any number of times
(possibly zero times).
*/

pub fn minimum_length(s: String) -> i32 {
    let chars: Vec<char> = s.chars().collect();
    let mut l = 0;
    let mut r = chars.len() - 1;
    while l < r && chars[l] == chars[r] {
        let matching = chars[l];
        while l < r && chars[l] == matching {
            l += 1;
        }
        while l < r && chars[r] == matching {
            r -= 1;
        }
        if l == r && chars[l] == matching {
            return 0;
        }
    }
    (r - l + 1) as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
