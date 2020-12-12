/*
You have the task of delivering some boxes from storage to their ports using only one ship.
However, this ship has a limit on the number of boxes and the total weight that it can carry.

You are given an array boxes, where boxes[i] = [ports​​i​, weighti], and three integers portsCount,
maxBoxes, and maxWeight.

    ports​​i is the port where you need to deliver the ith box and weightsi is the weight of the
    ith box.
    portsCount is the number of ports.
    maxBoxes and maxWeight are the respective box and weight limits of the ship.

The boxes need to be delivered in the order they are given. The ship will follow these steps:

    The ship will take some number of boxes from the boxes queue, not violating the maxBoxes and
    maxWeight constraints.
    For each loaded box in order, the ship will make a trip to the port the box needs to be
    delivered to and deliver it. If the ship is already at the correct port, no trip is needed, and
    the box can immediately be delivered.
    The ship then makes a return trip to storage to take more boxes from the queue.

The ship must end at storage after all the boxes have been delivered.

Return the minimum number of trips the ship needs to make to deliver all boxes to their respective
ports.
*/

/*
Constraints:

    1 <= boxes.length <= 105
    1 <= portsCount, maxBoxes, maxWeight <= 105
    1 <= ports​​i <= portsCount
    1 <= weightsi <= maxWeight

*/

pub fn box_delivering(boxes: Vec<Vec<i32>>, _: i32, max_boxes: i32, max_weight: i32) -> i32 {
    let max_boxes = max_boxes as usize;
    // dp[i] = trips to deliver boxes 0..i (eg, for boxes = [[1, 1],[2,2]], dp = [0, 2, 3])
    let mut dp = vec![0; boxes.len() + 1];
    let mut left = 0;
    let mut last_port = 0;
    let mut weight = 0;
    // initialize to 1 to include the trip back from the final port
    let mut port_changes = 1;
    for right in 0..boxes.len() {
        // load box[right]
        weight += boxes[right][1];
        if boxes[right][0] != last_port {
            port_changes += 1;
            last_port = boxes[right][0];
        }
        // unload the leftmost box from the current trip if over box limit
        while right - left >= max_boxes
            // over weight limit
            || weight > max_weight
            // or removing it would save a trip on the current run
            || (left < right && dp[left] == dp[left + 1])
        {
            weight -= boxes[left][1];
            if boxes[left + 1][0] != boxes[left][0] {
                port_changes -= 1;
            }
            left += 1;
        }
        // shipping boxes 0..=right is the cost of the current trip (left..=right) + 0..left
        dp[right + 1] = port_changes + dp[left];
    }
    *dp.last().unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
