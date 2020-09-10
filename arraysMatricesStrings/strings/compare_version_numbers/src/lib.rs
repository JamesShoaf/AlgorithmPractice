/* 
Compare two version numbers version1 and version2.
If version1 > version2 return 1
if version1 < version2 return -1
otherwise return 0.
*/

fn parse(next: Option<&str>) -> Option<i32> {
    if let Some(s) = next {
        if let Ok(val) = s.parse() {
            return Some(val);
        }
    }
    None
}

fn compare_version_numbers(ver1: String, ver2: String) -> i32 {
    let mut ver1 = ver1.split('.');
    let mut ver2 = ver2.split('.');
    let mut val1 = parse(ver1.next());
    let mut val2 = parse(ver2.next());

    loop {
        if val1.is_none() && val2.is_none() { break; }

        if let Some(int1) = val1 {
            if let Some(int2) = val2 {
                if int1 > int2 { return 1; }
                if int1 < int2 { return -1; }
            } else if int1 > 0 { return 1; }
        } else if let Some(int2) = val2 {
            if int2 > 0 { return -1; }
        }

        val1 = parse(ver1.next());
        val2 = parse(ver2.next());
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (String::from("0.1"), String::from("1.1"), -1),
            (String::from("1.0.1"), String::from("1"), 1),
            (String::from("7.5.2.4"), String::from("7.5.3"), -1),
            (String::from("1.01"), String::from("1.001"), 0),
            (String::from("1.0"), String::from("1.0.0"), 0),
            (String::from("1.a"), String::from("1.0"), 0),
            (String::from("1.a"), String::from("1.0.1"), -1),
            (String::from("1.0"), String::from("1.a"), 0),
            (String::from("1.0.1"), String::from("1.a"), 1),
        ];
        for (ver1, ver2, expected) in test_tuples {
            assert_eq!(compare_version_numbers(ver1, ver2), expected);
        }
    }
}
