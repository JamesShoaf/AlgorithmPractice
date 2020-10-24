pub fn fizz_buzz(n: i32) -> Vec<String> {
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

fn buzz (buzz: String, blank: String) -> Box<dyn Fn(i32) -> String> {
    Box::new(move |x| {
        if x % 5 == 0 {
            buzz.clone()
        } else {
            blank.clone()
        }
    })
}

fn fizz(n: i32) -> String {
    if n % 3 == 0 {
        buzz(String::from("FizzBuzz"), String::from("Fizz"))(n)
    } else {
        buzz(String::from("Buzz"), format!("{}", n))(n)
    }
}

pub fn lambda_buzz(n: i32) -> String {
    fizz(n)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        println!("{:?}", super::fizz_buzz(15));
        panic!();
    }
}
