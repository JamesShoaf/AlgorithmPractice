/* 
Given a rows x cols matrix mat, where mat[i][j] is either 0 or 1, return the number of special positions in mat.

A position (i,j) is called special if mat[i][j] == 1 and all other elements in row i and column j are 0 (rows and columns are 0-indexed).
*/

fn num_special(mat: Vec<Vec<i32>>) -> i32 {
    let rows = mat.len();
    if rows == 0 { return 0; }
    let cols = mat[0].len();
    if cols == 0 { return 0; }
    let mut count = 0;
    for i in 0..rows {
        if mat[i].iter().fold(0, |acc, &val| acc + val) == 1 {
            if let Some((j, _)) = mat[i].iter().enumerate().find(|&(_, &x)| x == 1) {
                let mut sum = 0;
                for k in 0..rows {
                    if mat[k][j] == 1 {
                        sum += 1;
                        if sum > 1 { break; }
                    }
                }
                if sum == 1 { count += 1;}
            }
        }
    }
    count
}
