/*
You are given an array of distinct integers arr and an array of integer arrays pieces, where the
integers in pieces are distinct. Your goal is to form arr by concatenating the arrays in pieces in
any order. However, you are not allowed to reorder the integers in each array pieces[i].

Return true if it is possible to form the array arr from pieces. Otherwise, return false.
*/

use std::collections::HashMap;

pub fn can_form_array(arr: Vec<i32>, pieces: Vec<Vec<i32>>) -> bool {
    if pieces.iter().map(|piece| piece.len()).sum::<usize>() != arr.len() {
        return false;
    }
    let pieces: HashMap<i32, Vec<i32>> = pieces
        .into_iter()
        .filter(|piece| !piece.is_empty())
        .map(|piece| (piece[0], piece))
        .collect();
    let mut i = 0;
    while i < arr.len() {
        if let Some(piece) = pieces.get(&arr[i]) {
            for j in 0..piece.len() {
                if arr[i] != piece[j] {
                    return false;
                }
                i += 1;
            }
        } else {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![85], vec![vec![85]], true),
            (vec![15, 88], vec![vec![88], vec![15]], true),
            (vec![49, 18, 16], vec![vec![16, 18, 49]], false),
            (
                vec![91, 4, 64, 78],
                vec![vec![78], vec![4, 64], vec![91]],
                true,
            ),
            (vec![1, 3, 5, 7], vec![vec![2, 4, 6, 8]], false),
        ];
        for (arr, pieces, expected) in test_tuples {
            assert_eq!(super::can_form_array(arr, pieces), expected);
        }
    }
}
