/*
There are 1000 buckets, one and only one of them is laced with a euphoriant, while the rest are
filled with water. They all look identical. If a pig drinks the euphoriant it will roll over on its
back and oink gleefully within 15 minutes, after which it will not be capable of further testing.
What is the minimum amount of pigs you need to figure out which bucket is laced within one hour?

Answer this question, and write an algorithm for the general case.
*/

/*
The trick to this problem is to maximize the amount of information each pig gives. One strategy to
do that is to determine the number of tests possible (T), then assign each bucket a number in base
T + 1. For each place in the maximum number, assign a pig. At round 1, that pig will drink from all
buckets with a 1 in that place. At round 2, if the pig is still sober, it will drink from all
buckets with a 2 in that place (and so on).
ex: 9 buckets, 2 tests, 2 pigs (A and B)
00 01 02 10 11 12 20 21 22
__ _B __ A_ AB A_ __ _B __

if pig B oinks in this round, only buckets 11, and 21 can be laced (since bucket 11 did not cause
pig A to oink). (Pig B traded off for maximum information)
In round 2, pig A drinks from bucket 21
00 01 02 10 11 12 20 21 22
XX __ XX XX XX XX XX A_ XX

Pig A also begins to oink. Bucket 21 contains the euphoriant.

Consider a low information alternative outcome where both pigs A and B are sober after the first
test:
00 01 02 10 11 12 20 21 22
__ XX _B XX XX XX A_ XX AB

The 4 remaining outcomes each have a unique combination of pigs associated with them.
*/

pub fn happy_pigs(buckets: i32, minutes_to_oink: i32, minutes_to_test: i32) -> i32 {
    let buckets = buckets as f64;
    let tests = (minutes_to_test / minutes_to_oink) as f64;
    if tests < 1.0 {
        return -1;
    }
    (buckets.ln() / (tests + 1.0).ln()).ceil() as i32
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
