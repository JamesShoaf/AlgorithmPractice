/*
You are working in a ball factory where you have n balls numbered from lowLimit up to highLimit
inclusive (i.e., n == highLimit - lowLimit + 1), and an infinite number of boxes numbered from
1 to infinity.

Your job at this factory is to put each ball in the box with a number equal to the sum of digits
of the ball's number. For example, the ball number 321 will be put in the box number
3 + 2 + 1 = 6 and the ball number 10 will be put in the box number 1 + 0 = 1.

Given two integers lowLimit and highLimit, return the number of balls in the box with the most
balls.
*/

pub fn count_balls(low_limit: i32, high_limit: i32) -> i32 {
    // 9s in each of 10 places = 90 for a 10 digit number
    let mut buckets = [0; 90];
    for mut num in low_limit..high_limit + 1 {
        let mut sum = 0;
        while num > 0 {
            sum += num % 10;
            num /= 10;
        }
        buckets[sum as usize] += 1;
    }
    *buckets.iter().max().unwrap_or(&0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
