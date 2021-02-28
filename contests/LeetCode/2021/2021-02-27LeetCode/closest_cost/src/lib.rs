/*
You would like to make dessert and are preparing to buy the ingredients. You have n ice cream
base flavors and m types of toppings to choose from. You must follow these rules when making
your dessert:

    There must be exactly one ice cream base.
    You can add one or more types of topping or have no toppings at all.
    There are at most two of each type of topping.

You are given three inputs:

    baseCosts, an integer array of length n, where each baseCosts[i] represents the price of
        the ith ice cream base flavor.
    toppingCosts, an integer array of length m, where each toppingCosts[i] is the price of one
        of the ith topping.
    target, an integer representing your target price for dessert.

You want to make a dessert with a total cost as close to target as possible.

Return the closest possible cost of the dessert to target. If there are multiple, return the lower one.
*/

fn backtrack(
    current: i32,
    topping_costs: &Vec<i32>,
    topping_index: usize,
    target: i32,
    res: &mut i32,
) {
    if current > target || topping_index == topping_costs.len() {
        let current_diff = (target - current).abs();
        let res_diff = (target - *res).abs();
        if current_diff < res_diff || (current_diff == res_diff && current < *res) {
            *res = current;
        }
        return;
    }
    backtrack(
        current + topping_costs[topping_index] * 2,
        topping_costs,
        topping_index + 1,
        target,
        res,
    );
    backtrack(
        current + topping_costs[topping_index],
        topping_costs,
        topping_index + 1,
        target,
        res,
    );
    backtrack(current, topping_costs, topping_index + 1, target, res);
}

pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
    let mut res = *base_costs.iter().min().unwrap();
    for cost in base_costs {
        backtrack(cost, &topping_costs, 0, target, &mut res);
    }
    res
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
