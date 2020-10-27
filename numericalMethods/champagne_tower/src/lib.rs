/* 
We stack glasses in a pyramid, where the first row has 1 glass, the second row has 2 glasses, and
so on until the 100th row.  Each glass holds one cup of champagne.

Then, some champagne is poured into the first glass at the top.  When the topmost glass is full,
any excess liquid poured will fall equally to the glass immediately to the left and right of it.
When those glasses become full, any excess champagne will fall equally to the left and right of
those glasses, and so on.  (A glass at the bottom row has its excess champagne fall on the floor.)

For example, after one cup of champagne is poured, the top most glass is full.  After two cups of
champagne are poured, the two glasses on the second row are half full.  After three cups of
champagne are poured, those two cups become full - there are 3 full glasses total now.  After four
cups of champagne are poured, the third row has the middle glass half full, and the two outside
glasses are a quarter full, as pictured below.
*/

pub fn champagne_tower(poured: i32, query_row: i32, query_glass: i32) -> f64 {
    assert!(poured >= 0, "poured must be a non-negative integer");
    assert!(query_row >= 0 && query_row <= 99, "row must be between 0 and 99");
    assert!(query_glass >= 0 && query_glass <= query_row, "requested glass {} of row {} is out of
        bounds", query_glass, query_row);
    let (query_row, query_glass) = (query_row as usize, query_glass as usize);
    let mut tower: Vec<Vec<f64>> = Vec::new();
    for i in 1..=query_row + 1 {
        tower.push(vec![0.0; i]);
    }
    tower[0][0] = poured as f64;
    for i in 0..query_row {
        for j in 0..=i {
            if tower[i][j] > 1.0 {
                tower[i + 1][j] += (tower[i][j] - 1.0) / 2.0;
                tower[i + 1][j + 1] += (tower[i][j] - 1.0) / 2.0;
            }
        }
    }
    if tower[query_row][query_glass] >= 1.0 { 1.0 }
    else { tower[query_row][query_glass] }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (0, 0, 0, 0.0),
            (1, 0, 0, 1.0),
            (1, 1, 1, 0.0),
            (2, 1, 1, 0.5),
            (4, 2, 0, 0.25),
            (4, 2, 1, 0.5),
            (4, 2, 2, 0.25),
            (100000009, 33, 17, 1.0),
        ];
        for (poured, row, glass, expected) in test_tuples {
            assert_eq!(champagne_tower(poured, row, glass), expected);
        }
    }
}
