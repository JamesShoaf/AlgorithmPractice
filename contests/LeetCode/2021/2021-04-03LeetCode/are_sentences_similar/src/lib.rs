/*
A sentence is a list of words that are separated by a single space with no leading or trailing spaces. For example, "Hello World", "HELLO", "hello world hello world" are all sentences. Words consist of only uppercase and lowercase English letters.

Two sentences sentence1 and sentence2 are similar if it is possible to insert an arbitrary sentence (possibly empty) inside one of these sentences such that the two sentences become equal. For example, sentence1 = "Hello my name is Jane" and sentence2 = "Hello Jane" can be made equal by inserting "my name is" between "Hello" and "Jane" in sentence2.

Given two sentences sentence1 and sentence2, return true if sentence1 and sentence2 are similar. Otherwise, return false.
*/

fn helper(split1: Vec<&str>, split2: Vec<&str>) -> bool {
    if split1.len() < split2.len() {
        return helper(split2, split1);
    }
    let mut a = 0;
    let mut b = 0;
    // skip over to first insertion
    while a < split1.len() && b < split2.len() && split1[a] == split2[b] {
        a += 1;
        b += 1;
    }
    if b == split2.len() {
        return true;
    }
    // insert words into b until another match with a is found
    while a < split1.len() && b < split2.len() && split1[a] != split2[b] {
        a += 1;
        if a == split1.len() {
            return false;
        }
    }
    // skip to end or next mismatch
    while a < split1.len() && b < split2.len() && split1[a] == split2[b] {
        a += 1;
        b += 1;
    }
    // if the end of a is reached or the end of b is reached and all insertions are postfix, return true
    a == split1.len() && b == split2.len()
}

pub fn are_sentences_similar(sentence1: String, sentence2: String) -> bool {
    if sentence1 == sentence2 {
        return true;
    }
    let split1: Vec<&str> = sentence1.split_whitespace().collect();
    let split2: Vec<&str> = sentence2.split_whitespace().collect();
    return helper(split1, split2);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
