/*
You own a Goal Parser that can interpret a string command. The command consists of an alphabet of
"G", "()" and/or "(al)" in some order. The Goal Parser will interpret "G" as the string "G", "()"
as the string "o", and "(al)" as the string "al". The interpreted strings are then concatenated in
the original order.

Given the string command, return the Goal Parser's interpretation of command.
*/

pub fn interpret(command: String) -> String {
    let mut res = String::new();
    let mut prev: char = 'z';
    for c in command.chars() {
        match c {
            'G' | 'a' => res.push(c),
            '(' => prev = '(',
            'l' => {
                prev = 'l';
                res.push('l');
            }
            ')' if prev == '(' => res.push('o'),
            _ => (),
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![(String::from("G()(al)"), String::from("Goal"))];
        for (command, expected) in test_tuples {
            assert_eq!(interpret(command), expected);
        }
    }
}
