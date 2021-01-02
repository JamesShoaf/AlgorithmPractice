/*
You are given four integers, m, n, introvertsCount, and extrovertsCount. You have an m x n grid,
and there are two types of people: introverts and extroverts. There are introvertsCount introverts
and extrovertsCount extroverts.

You should decide how many people you want to live in the grid and assign each of them one grid
cell. Note that you do not have to have all the people living in the grid.

The happiness of each person is calculated as follows:

Introverts start with 120 happiness and lose 30 happiness for each neighbor (introvert orextrovert).
Extroverts start with 40 happiness and gain 20 happiness for each neighbor (introvert or extrovert).

Neighbors live in the directly adjacent cells north, east, south, and west of a person's cell.

The grid happiness is the sum of each person's happiness. Return the maximum possible grid happiness.

Constraints:

    1 <= m, n <= 5
    0 <= introvertsCount, extrovertsCount <= min(m * n, 6)

*/

struct Consts {
    rows: usize,
    cols: usize,
    oob_index: usize,
    ones: usize,
    north_bit: usize,
    int_base: i32,
    int_bonus: i32,
    ext_base: i32,
    ext_bonus: i32,
}

fn generate_constants(
    &rows: &usize,
    &cols: &usize,
    int_base: i32,
    int_bonus: i32,
    ext_base: i32,
    ext_bonus: i32,
) -> Consts {
    Consts {
        rows,
        cols,
        oob_index: rows * cols,
        ones: 2_usize.pow(cols as u32) - 1,
        north_bit: if cols > 0 { 1 << cols - 1 } else { 0 },
        int_base,
        int_bonus,
        ext_base,
        ext_bonus,
    }
}

enum Neighbor {
    Introvert,
    Extrovert,
    None,
}

#[derive(Clone)]
struct Masks {
    int_mask: usize,
    ext_mask: usize,
}

fn get_next_masks(masks: &Masks, next: &Neighbor, consts: &Consts) -> Masks {
    let mut updated = masks.clone();
    updated.int_mask <<= 1;
    updated.int_mask &= consts.ones;
    updated.ext_mask <<= 1;
    updated.ext_mask &= consts.ones;
    match next {
        Neighbor::Introvert => updated.int_mask += 1,
        Neighbor::Extrovert => updated.ext_mask += 1,
        _ => (),
    }
    updated
}

struct NeighborStats {
    neighbor_count: i32,
    neighbor_diff: i32,
}

fn get_neighbor_stats(i: &usize, masks: &Masks, consts: &Consts) -> NeighborStats {
    let mut neighbor_count = 0;
    let mut neighbor_diff = 0;
    if i % consts.cols != 0 && masks.int_mask & 1 != 0 {
        neighbor_count += 1;
        neighbor_diff += consts.int_bonus;
    }
    if masks.int_mask & consts.north_bit != 0 {
        neighbor_count += 1;
        neighbor_diff += consts.int_bonus;
    }
    if i % consts.cols != 0 && masks.ext_mask & 1 != 0 {
        neighbor_count += 1;
        neighbor_diff += consts.ext_bonus;
    }
    if masks.ext_mask & consts.north_bit != 0 {
        neighbor_count += 1;
        neighbor_diff += consts.ext_bonus;
    }
    NeighborStats {
        neighbor_count,
        neighbor_diff,
    }
}

fn get_score(i: &usize, neighbor: &Neighbor, masks: &Masks, consts: &Consts) -> i32 {
    let stats = get_neighbor_stats(i, masks, consts);
    match neighbor {
        Neighbor::Introvert => {
            consts.int_base + stats.neighbor_count * consts.int_bonus + stats.neighbor_diff
        }
        Neighbor::Extrovert => {
            consts.ext_base + stats.neighbor_count * consts.ext_bonus + stats.neighbor_diff
        }
        Neighbor::None => 0,
    }
}

type DP5 = Vec<Vec<Vec<Vec<Vec<Option<i32>>>>>>;

// index: current_row * cols + current_col - convert 2d to 1d
// memo[index][rem_introverts][rem_extroverts][int_mask][ext_mask]
// dimensions: max_index x introvert_states x extrovert_states x
fn initalize_dp(counts: &Counts, consts: &Consts) -> DP5 {
    vec![
        vec![
            vec![
                vec![vec![None; 2_usize.pow(consts.cols as u32)]; 2_usize.pow(consts.cols as u32)];
                counts.rem_ext + 1
            ];
            counts.rem_int + 1
        ];
        consts.rows * consts.cols
    ]
}

struct Counts {
    rem_int: usize,
    rem_ext: usize,
}

fn get_total_score(
    i: &usize,
    neighbor: Neighbor,
    counts: &mut Counts,
    masks: &Masks,
    memo: &mut DP5,
    consts: &Consts,
) -> i32 {
    let next_masks = get_next_masks(masks, &neighbor, consts);
    get_score(i, &neighbor, &masks, consts) + dfs(i + 1, counts, next_masks, memo, consts)
}

use std::cmp;

fn dfs(i: usize, counts: &mut Counts, masks: Masks, memo: &mut DP5, consts: &Consts) -> i32 {
    if i == consts.oob_index {
        return 0;
    }
    if let Some(prev) = memo[i][counts.rem_int][counts.rem_ext][masks.int_mask][masks.ext_mask] {
        return prev;
    }
    let mut res = get_total_score(&i, Neighbor::None, counts, &masks, memo, consts);
    if counts.rem_int > 0 {
        counts.rem_int -= 1;
        res = cmp::max(
            res,
            get_total_score(&i, Neighbor::Introvert, counts, &masks, memo, consts),
        );
        counts.rem_int += 1;
    }
    if counts.rem_ext > 0 {
        counts.rem_ext -= 1;
        res = cmp::max(
            res,
            get_total_score(&i, Neighbor::Extrovert, counts, &masks, memo, consts),
        );
        counts.rem_ext += 1;
    }
    memo[i][counts.rem_int][counts.rem_ext][masks.int_mask][masks.ext_mask] = Some(res);
    res
}

pub fn get_max_grid_happiness(rows: i32, cols: i32, int_count: i32, ext_count: i32) -> i32 {
    let rows = rows as usize;
    let cols = cols as usize;
    let mut counts = Counts {
        rem_int: int_count as usize,
        rem_ext: ext_count as usize,
    };
    let consts = generate_constants(&rows, &cols, 120, -30, 40, 20);
    let mut memo: DP5 = initalize_dp(&counts, &consts);
    let masks = Masks {
        int_mask: 0,
        ext_mask: 0,
    };
    let res = dfs(0, &mut counts, masks, &mut memo, &consts);
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let test_tuples = vec![
            (0, 1, 1, 1, 0),
            (1, 0, 1, 1, 0),
            (1, 1, 0, 0, 0),
            (1, 1, 1, 0, 120),
            (1, 1, 0, 1, 40),
            (3, 1, 2, 1, 260),
            (1, 3, 2, 1, 260),
            (2, 3, 1, 2, 240),
            (5, 5, 6, 6, 1240),
        ];
        for (rows, cols, int_count, ext_count, expected) in test_tuples {
            assert_eq!(
                get_max_grid_happiness(rows, cols, int_count, ext_count),
                expected
            );
        }
    }
}
