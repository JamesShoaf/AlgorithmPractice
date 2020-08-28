/* 
Given a function rand7 which generates a uniform random integer in the range 1 to 7, write a
function rand10 which generates a uniform random integer in the range 1 to 10.
*/

use rand::{Rng, thread_rng };

fn rand7() -> i32 {
    let mut rng = thread_rng();
    rng.gen_range(1, 8)
}

// converts 2 calls to rand7 to a roughly even distribution of 1 to 10
// (1 has only 80% of the probability other numbers have)
fn rand10() -> i32 {
    ((rand7() - 1) * 7 + rand7()) / 5 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        for _ in 0..100 {
            let rand = rand10();
            if rand < 1 { panic!("Random output less than 1"); }
            if rand > 10 { panic!("Random output greater than 10"); }
        }
    }
}
