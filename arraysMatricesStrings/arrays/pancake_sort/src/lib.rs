/* 
Given an array of integers A, We need to sort the array performing a series of pancake flips.

In one pancake flip we do the following steps:

    Choose an integer k where 0 <= k < A.length.
    Reverse the sub-array A[0...k].

For example, if A = [3,2,1,4] and we performed a pancake flip choosing k = 2, we reverse the
sub-array [3,2,1], so A = [1,2,3,4] after the pancake flip at k = 2.

Return an array of the k-values of the pancake flips that should be performed in order to sort A.

All integers in A are unique (i.e. A is a permutation of the integers from 1 to A.length).
1 <= A.length <= 100
1 <= A[i] <= A.length
*/

fn flip(i: usize, j: usize, mut a: Vec<i32>) -> Vec<i32> {
    for k in 0..(j + 1) / 2 {
        a.swap(k, j - k);
    }
    for k in 0..(i + 1) / 2 {
        a.swap(k, i - k);
    }
    return pancake_sort(a);
}

fn pancake_sort(a: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = a.len();
    for index in (0..a.len()).rev() {
        if a[index] != (index + 1) as i32 {
            i = index;
            break;
        }
    }
    for index in 0..i {
        if a[index] == (i + 1) as i32 {
            j = index;
            break;
        }
    }
    if i == 0 || j == a.len() { return Vec::new(); }
    let mut output = Vec::new();
    if j > 0 { output.push((j + 1) as i32); }
    output.push((i + 1) as i32);
    let mut rest = flip(i, j, a);
    output.append(&mut rest);
    output
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (vec![5], Vec::new()),
            (vec![1, 2, 3], Vec::new()),
            (vec![3, 2, 4, 1], vec![3, 4, 2, 3, 2]),
            (vec![1, 5, 3, 4, 2], vec![2, 5, 2, 4, 2, 3, 2]),
            (vec![1, 2, 3, 7, 5, 6, 4, 8, 9], vec![4, 7, 2, 6, 4, 5, 4]),
        ];
        for (a, expected) in test_tuples {
            assert_eq!(pancake_sort(a), expected);
        }
    }
}
