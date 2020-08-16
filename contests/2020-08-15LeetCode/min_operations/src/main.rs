/* 
You have an array arr of length n where arr[i] = (2 * i) + 1 for all valid values of i (i.e. 0 <= i < n).

In one operation, you can select two indices x and y where 0 <= x, y < n and subtract 1 from arr[x] and
add 1 to arr[y] (i.e. perform arr[x] -=1 and arr[y] += 1). The goal is to make all the elements of the
array equal. It is guaranteed that all the elements of the array can be made equal using some operations.

Given an integer n, the length of the array. Return the minimum number of operations needed to make all
the elements of arr equal.
*/


fn min_operations(n: i32) -> i32 {
    let mut nums = Vec::new();
    for i in 0..n { nums.push(2*i + 1); }
    let mut sum = 0;
    for num in nums {
        if num >= n { return sum; }
        sum += n - num;
    }
    sum
}

fn main() {
    println!("Hello, world!");
}
