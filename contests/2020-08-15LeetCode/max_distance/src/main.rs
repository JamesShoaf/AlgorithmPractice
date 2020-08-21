/* 
In universe Earth C-137, Rick discovered a special form of magnetic force between two balls if they
are put in his new invented basket. Rick has n empty baskets, the ith basket is at position[i],
Morty has m balls and needs to distribute the balls into the baskets such that the minimum magnetic
force between any two balls is maximum.

Rick stated that magnetic force between two different balls at positions x and y is |x - y|.

Given the integer array position and the integer m. Return the required force.
*/

fn can_place_balls(position: &Vec<i32>, prev_index: usize, remaining: i32, min_force: i32) -> bool {
    if remaining == 1 { return true; }
    for i in prev_index + 1..position.len() {
        if position[i] - position[prev_index] >= min_force {
            println!("basket {} at position {}, {} balls remaining", position[i], i, remaining - 1);
            return can_place_balls(&position, i, remaining - 1, min_force);
        }
    }
    false
}

pub fn max_distance(mut position: Vec<i32>, m: i32) -> i32 {
    position.sort();
    println!("{:?}", position);
    let first_ball = position[0];
    println!("first ball {}", first_ball);
    let mut max_force = 0;
    for i in 1..position.len() {
        let next_max = position[i] - first_ball;
        println!("next max {}", next_max);
        if can_place_balls(&position, i, m - 1, next_max) {
            println!("placing second ball at position {}, remaining {}, next_max {}", i, m - 2, next_max);
            max_force = next_max;
        } else { break; }
    }
    max_force
}

fn main() {
    let test = vec![5,4,3,2,1,1000000000];
    let m = 2;
    // let test_tuples = vec![
    //     (vec![1, 2, 3, 4, 7], 3),
    //     (vec![5,4,3,2,1,1000000000], 2),
    // ];
    println!("{}", max_distance(test, m));
}
