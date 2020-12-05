/*
You have a long flowerbed in which some of the plots are planted, and some are not. However,
flowers cannot be planted in adjacent plots.

Given an integer array flowerbed containing 0's and 1's, where 0 means empty and 1 means not empty,
and an integer n, return if n new flowers can be planted in the flowerbed without violating the
no-adjacent-flowers rule.
*/

fn calculate_room(zeros_in_run: &mut i32, room_in_flowerbed: &mut i32) {
    *room_in_flowerbed += (*zeros_in_run - 1) / 2;
    *zeros_in_run = 0;
}

pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
    let mut zeros_in_run = 1;
    let mut room_in_flowerbed = 0;
    for i in 0..flowerbed.len() {
        if flowerbed[i] == 0 {
            zeros_in_run += 1;
        } else {
            calculate_room(&mut zeros_in_run, &mut room_in_flowerbed);
        }
    }
    zeros_in_run += 1;
    calculate_room(&mut zeros_in_run, &mut room_in_flowerbed);
    room_in_flowerbed >= n
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (Vec::new(), 0, true),
            (Vec::new(), 1, false),
            (vec![0], 0, true),
            (vec![0], 1, true),
            (vec![0], 2, false),
            (vec![0, 0], 1, true),
            (vec![0, 0], 2, false),
            (vec![0, 0, 0], 1, true),
            (vec![0, 0, 0], 2, true),
            (vec![0, 0, 0], 3, false),
            (vec![1, 0, 1], 0, true),
            (vec![1, 0, 1], 1, false),
            (vec![0, 0, 0, 0], 2, true),
            (vec![0, 0, 0, 0], 3, false),
            (vec![0, 0, 1, 0, 0, 0, 1, 0, 0], 3, true),
            (vec![0, 0, 1, 0, 0, 0, 1, 0, 0], 4, false),
        ];
        for (flowerbed, n, expected) in test_tuples {
            assert_eq!(can_place_flowers(flowerbed, n), expected);
        }
    }
}
