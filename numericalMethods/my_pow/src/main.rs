struct Solution {}

impl Solution {
    // x^n = sum_(n=0)^∞ (n^m log^m(x))/(m!)
    pub fn my_pow_taylor(x: f64, n: i32) -> f64 {
        if n == 0 { return x }
        if x == 1.0 { return 1.0 }
        let n_log_x: f64 = n as f64 * x.abs().ln();
        let neg: f64 = if n % 2 == 0 { 1.0 } else { -1.0 };
        const ERROR_THRESHOLD: f64 = 0.000001;
        let mut m: f64 = 0.0;
        let mut m_fac: f64 = 1.0;
        let mut n_log_x_m: f64 = 1.0;
        let mut last_term: f64 = 1.0;
        let mut output: f64 = last_term;
        while last_term.abs() >= ERROR_THRESHOLD {
            println!("output: {}", output);
            m += 1.0;
            println!("m: {}", m);
            m_fac *= m;
            println!("m_fac: {}", m_fac);
            n_log_x_m *= n_log_x;
            println!("n_log_x_m: {}", n_log_x_m);
            last_term = (n_log_x_m) / m_fac;
            println!("last_term: {}", last_term);
            output += last_term;
        }
        output * neg
    }

    pub fn my_pow(x: f64, n: i32) -> f64 {
        if n == 0 || x == 1.0 { return 1.0 }
        let n_pos = n.abs();
        let mut bit = 1;
        let mut x_pow = x;
        let mut output: f64 = 1.0;
        for x in 0..32 {
            if n_pos & bit != 0 {
                output *= x_pow;
            }
            if x != 31 {
                bit <<= 1;
                x_pow *= x_pow;
            }
        }
        if n < 0 { 
            if output.is_finite() { return 1.0 / output; }
            return -std::f64::MIN_POSITIVE;
        }
        output
    }
}

fn main() {
    let x: f64 = 0.9999999999999999999999999999999;
    let n: i32 = 2147483647;
    let pow = Solution::my_pow(x, n);
    println!("{}", pow);
}
