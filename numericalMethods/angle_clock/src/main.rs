struct Solution {}

impl Solution {
    pub fn angle_clock(hour: i32, minutes: i32) -> f64 {
        let minute_angle: f64 = minutes as f64 * 6_f64;
        let hour_angle: f64 = 30_f64 * (hour as f64 % 12_f64) + (minutes as f64 / 2_f64);
        let angle:f64 = (minute_angle - hour_angle).abs();
        if angle <= 180_f64 {angle} else {360_f64 - angle}
    }
}

fn main() {
    assert_eq!(Solution::angle_clock(12, 30), 165_f64);
    assert_eq!(Solution::angle_clock(6, 30), 15_f64);
}
