/*
You are assigned to put some amount of boxes onto one truck. You are given a 2D array boxTypes,
where boxTypes[i] = [numberOfBoxesi, numberOfUnitsPerBoxi]:

    numberOfBoxesi is the number of boxes of type i.
    numberOfUnitsPerBoxi is the number of units in each box of the type i.

You are also given an integer truckSize, which is the maximum number of boxes that can be put on
the truck. You can choose any boxes to put on the truck as long as the number of boxes does not
exceed truckSize.

Return the maximum total number of units that can be put on the truck.
*/

use std::cmp::{self, Reverse};

pub fn maximum_units(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
    box_types.sort_unstable_by_key(|bx| (Reverse(bx[1]), bx[0]));
    box_types
        .into_iter()
        .fold((truck_size, 0), |(capacity, count), bx| {
            let boxes_taken = cmp::min(capacity, bx[0]);
            (capacity - boxes_taken, count + bx[1] * boxes_taken)
        })
        .1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
