/*
You are given a string s of even length. Split this string into two halves of equal lengths, and let a be the first half and b be the second half.

Two strings are alike if they have the same number of vowels ('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'). Notice that s contains uppercase and lowercase letters.

Return true if a and b are alike. Otherwise, return false.
*/

pub fn halves_are_alike(s: String) -> bool {
    let chars: Vec<char> = s.to_lowercase().chars().collect();
    let mut left_count = 0;
    let mut right_count = 0;
    for (i, c) in chars.iter().enumerate() {
        match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                if i < chars.len() / 2 {
                    left_count += 1;
                } else {
                    right_count += 1;
                }
            }
            _ => (),
        }
    }
    left_count == right_count
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
