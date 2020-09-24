/* 
There are N gas stations along a circular route, where the amount of gas at station i is gas[i].

You have a car with an unlimited gas tank and it costs cost[i] of gas to travel from station i to
its next station (i+1). You begin the journey with an empty tank at one of the gas stations.

Return the starting gas station's index if you can travel around the circuit once in the clockwise
direction, otherwise return -1.

Note:

    If there exists a solution, it is guaranteed to be unique.
    Both input arrays are non-empty and have the same length.
    Each element in the input arrays is a non-negative integer.

*/

pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
    let (mut total_gas, mut total_cost, mut start, mut tank) = (0, 0, 0, 0);
    for i in 0..gas.len() {
        total_gas += gas[i];
        total_cost += cost[i];
        tank += gas[i] - cost[i];
        if tank < 0 {
            start = i + 1;
            tank = 0;
        }
    }
    if total_gas < total_cost {
        return -1;
    } 
    start as i32
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![1, 2, 3, 4, 5], vec![3, 4, 5, 1, 2], 3),
            (vec![2, 3, 4], vec![3, 4, 3], -1),
        ];
        for (gas, cost, expected) in test_tuples {
            assert_eq!(can_complete_circuit(gas, cost), expected);
        }
    }
}
