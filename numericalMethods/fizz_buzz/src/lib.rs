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



fn lambda_buzz(n: i32) -> String {
    fn fizz(n: i32) -> Box<dyn Fn(i32) -> String> {
        if n % 3 == 0 {
            Box::new(|x| {
                if x % 5 == 0 {
                    String::from("FizzBuzz")
                } else { String::from("Fizz") }
            })
        } else {
            Box::new(|x| {
                if x % 5 == 0 {
                    String::from("Buzz")
                } else {
                    format!("{}", x)
                }
            })
        }
    }
    fizz(n)(n)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{:?}", super::fizz_buzz(15));
        panic!();
    }
}
