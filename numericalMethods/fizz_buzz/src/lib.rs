fn fizz_buzz(n: i32) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    for i in 1..=n {
        let mut string = String::new();
        if i % 3 == 0 { string += "Fizz"; }
        if i % 5 == 0 { string += "Buzz"; }
        if string.len() == 0 { string += &format!("{}", i); }
        output.push(string);
    }
    output
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{:?}", super::fizz_buzz(15));
        panic!();
    }
}
