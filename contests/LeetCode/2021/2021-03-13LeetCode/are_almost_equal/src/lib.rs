/*
You are given two strings s1 and s2 of equal length. A string swap is an operation where you choose
two indices in a string (not necessarily different) andswap the characters at these indices.

Return true if it is possible to make both strings equal by performing at most one string swap on
exactly one of the strings. Otherwise, return false.
*/

pub fn are_almost_equal(s1: String, s2: String) -> bool {
    if s1.len() != s2.len() {
        return false;
    }
    let diffs: Vec<(char, char)> = s1.chars().zip(s2.chars()).filter(|(c, d)| c != d).collect();
    if diffs.is_empty() {
        return true;
    }
    if diffs.len() != 2 {
        return false;
    }
    diffs[0].0 == diffs[1].1 && diffs[1].0 == diffs[0].1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
