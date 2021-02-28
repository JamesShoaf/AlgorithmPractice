/*
There are n cars traveling at different speeds in the same direction along a one-lane road.
You are given an array cars of length n, where cars[i] = [positioni, speedi] represents:

    positioni is the distance between the ith car and the beginning of the road in meters.
    It is guaranteed that positioni < positioni+1.
    speedi is the initial speed of the ith car in meters per second.

For simplicity, cars can be considered as points moving along the number line. Two cars
collide when they occupy the same position. Once a car collides with another car, they
unite and form a single car fleet. The cars in the formed fleet will have the same position
and the same speed, which is the initial speed of the slowest car in the fleet.

Return an array answer, where answer[i] is the time, in seconds, at which the ith car
collides with the next car, or -1 if the car does not collide with the next car. Answers
within 10^-5 of the actual answers are accepted.
*/

// Because car fleets occupy the same space and have the slower car's speed, the fleet is
// effectively the front car alone, and the rear car effectively vanishes.
// Because cars can only collide with cars in front of them moving slower, the problem can be
// restated as "when does the current car collide with a slower forward car which has not
// already vanished" which lends itself to a stack-based solution

pub fn get_collision_times(cars: Vec<Vec<i32>>) -> Vec<f64> {
    let mut res: Vec<f64> = Vec::new();
    // (position, speed, collision time)
    let mut stack: Vec<(f64, f64, f64)> = Vec::new();
    // build stack of slow cars from the front backward
    for i in (0..cars.len()).rev() {
        let position = cars[i][0] as f64;
        let speed = cars[i][1] as f64;
        loop {
            if let Some(&(top_pos, top_speed, top_time)) = stack.last() {
                let collision_time = (top_pos - position) / (speed - top_speed);
                // pop fast cars and cars that will have already collided from the stack
                if speed <= top_speed || collision_time >= top_time {
                    stack.pop();
                    continue;
                }
                // push the current car's position, speed, and collision time to the stack
                stack.push((position, speed, collision_time));
                // and the time to the result
                res.push(collision_time);
                break;
            } else {
                // if no such slow cars exist, the collision time is infinity
                stack.push((position, speed, f64::MAX));
                // and note that the car does not collide
                res.push(-1.0);
                break;
            }
        }
    }
    // put results in left to right order
    res.reverse();
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
