/* 
You are playing the following Bulls and Cows game with your friend: You write down a number and
ask your friend to guess what the number is. Each time your friend makes a guess, you provide a
hint that indicates how many digits in said guess match your secret number exactly in both digit
and position (called "bulls") and how many digits match the secret number but locate in the
wrong position (called "cows"). Your friend will use successive guesses and hints to eventually
derive the secret number.

Write a function to return a hint according to the secret number and friend's guess, use A to
indicate the bulls and B to indicate the cows. 

Both the secret number and friend's guess may contain duplicate digits.
You may assume that the secret number and your friend's guess only contain digits, and their
lengths are always equal.
*/

fn counts(s: &str) -> [usize; 10] {
    let mut counts = [0; 10];
    for c in s.chars() {
        if let Some(num) = c.to_digit(10) {
            counts[num as usize] += 1;
        }
    }
    counts
}

fn get_hint(secret: String, guess: String) -> String {
    use std::cmp::min;
    let bulls = secret.chars().zip(guess.chars())
        .fold(0, |acc, (s, g)| if s == g {acc + 1} else { acc });
    let cows = counts(&secret).iter().zip(counts(&guess).iter())
        .fold(0, |acc, (s, g)| acc + min(s, g)) - bulls;
    format!("{}A{}B", bulls, cows)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("1807"), String::from("7810"), String::from("1A3B")),
            (String::from("1123"), String::from("0111"), String::from("1A1B")),
        ];
        for (secret, guess, expected) in test_tuples {
            assert_eq!(get_hint(secret, guess), expected);
        }
    }
}
