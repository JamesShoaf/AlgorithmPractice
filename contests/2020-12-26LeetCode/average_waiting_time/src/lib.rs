/*
There is a restaurant with a single chef. You are given an array customers, where customers[i] = [arrivali, timei]:

    arrivali is the arrival time of the ith customer. The arrival times are sorted in non-decreasing order.
    timei is the time needed to prepare the order of the ith customer.

When a customer arrives, he gives the chef his order, and the chef starts preparing it once he is idle. The customer waits till the chef finishes preparing his order. The chef does not prepare food for more than one customer at a time. The chef prepares food for customers in the order they were given in the input.

Return the average waiting time of all customers. Solutions within 10-5 from the actual answer are considered accepted.
*/

use std::cmp;

pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let total_customers = customers.len() as f64;
    customers
        .into_iter()
        .scan(0, |chef_free: &mut u64, customer: Vec<i32>| {
            let (arrival_time, order_duration) = (customer[0], customer[1]);
            *chef_free = cmp::max(*chef_free, arrival_time as u64) + order_duration as u64;
            Some(*chef_free - arrival_time as u64)
        })
        .sum::<u64>() as f64
        / total_customers
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
