/* 
You are given a string text of words that are placed among some number of spaces. Each word consists of one or more lowercase English letters and are separated by at least one space. It's guaranteed that text contains at least one word.

Rearrange the spaces so that there is an equal number of spaces between every pair of adjacent words and that number is maximized. If you cannot redistribute all the spaces equally, place the extra spaces at the end, meaning the returned string should be the same length as text.

Return the string after rearranging the spaces.
*/

pub fn reorder_spaces(text: String) -> String {
    let spaces_count = text.chars().fold(0, |acc, c| if c == ' ' { acc + 1 } else { acc });
    let word_count = text.split_whitespace().collect::<Vec<&str>>().len();
    if word_count == 0 { return text; }
    let mut split = text.split_whitespace();
    let first_word = String::from(split.next().unwrap());
    if word_count == 1 {
        return (0..spaces_count).fold(first_word, |acc, _| acc + " ");
    }
    let mut new = split.fold(first_word, |mut acc, word| {
        for _ in (0..spaces_count/(word_count - 1)) {
            acc += " ";
        }
        acc += word;
        acc
    });
    for _ in (0..spaces_count % (word_count - 1)) {
        new += " ";
    }
    new
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("  this   is  a sentence "), String::from("this   is   a   sentence")),
            (String::from(" practice   makes   perfect"), String::from("practice   makes   perfect ")),
            (String::from("hello   world"), String::from("hello   world")),
            (String::from("  walks  udp package   into  bar a"), String::from("walks  udp  package  into  bar  a ")),
            (String::from("a"), String::from("a")),
            (String::from("  a"), String::from("a  ")),
        ];
        for (text, expected) in test_tuples {
            assert_eq!(reorder_spaces(text), expected);
        }
    }
}
