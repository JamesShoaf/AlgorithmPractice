/* 
Given a rows x cols matrix mat, where mat[i][j] is either 0 or 1, return the number of special positions in mat.

A position (i,j) is called special if mat[i][j] == 1 and all other elements in row i and column j are 0 (rows and columns are 0-indexed).
*/

pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    (0..mat.len()).map(|i| {
        if mat[i].iter().sum::<i32>() == 1 {
            let j = mat[i].iter().position(|&x| x == 1).unwrap();
            if (0..mat.len()).map(|k| mat[k][j]).sum::<i32>() == 1 {
                return 1;
            }
        }
        0
    }).sum()
}