/* 
Given a list of non-overlapping axis-aligned rectangles rects, write a function pick which randomly
and uniformily picks an integer point in the space covered by the rectangles.

Note:

    1. An integer point is a point that has integer coordinates. 
    2. A point on the perimeter of a rectangle is included in the space covered by the rectangles. 
    3. ith rectangle = rects[i] = [x1,y1,x2,y2], where [x1, y1] are the integer coordinates of the
    bottom-left corner, and [x2, y2] are the integer coordinates of the top-right corner.
    4. length and width of each rectangle does not exceed 2000.
    5. 1 <= rects.length <= 100
    6. pick return a point as an array of integer coordinates [p_x, p_y]
    7. ick is called at most 10000 times.

*/

/*
new has O(n) time complexity, and pick has O(log(n)) time complexity
Uses O((h + w)n) space to map all points on perimeter to map all points on perimeter to ensure
adjacent rectangles do not have overlapping perimeters double-counted
Maps interior point counts for each rectangle, then uses binary search to choose the appropriate one.

If double-selection were not a factor, the binary search could be used for perimeter points as well,
reducing space complexity to O(n)
*/

struct Solution {
    // rects[i] = [x1, y1, x2, y2]
    rects: Vec<Vec<i32>>,
    // perimeter[i] = [x, y]
    perimeter: Vec<(i32, i32)>,
    perimeter_count: i32,
    // interior[i] = (count(rects[usize]), usize))
    interior: Vec<(i32, usize)>,
    point_count: i32,
}

impl Solution {
    fn new(rects: Vec<Vec<i32>>) -> Self {
        use std::collections::HashSet;
        let mut point_count: i32 = 0;
        let mut perimeter: HashSet<(i32, i32)> = HashSet::new();
        let mut interior: Vec<(i32, usize)> = Vec::new();
        for (i, rect) in rects.iter().enumerate() {
            let (x1, y1, x2, y2) = (rect[0], rect[1], rect[2], rect[3]);
            // add perimeter points to set
            for x in x1..=x2 {
                perimeter.insert((x, y1));
                perimeter.insert((x, y2));
            }
            for y in y1..=y2 {
                perimeter.insert((x1, y));
                perimeter.insert((x2, y));
            }
            // if there are interior points, push the count and the index of the current rect
            if y2 - y1 > 1 && x2 - x1 > 1 {
                point_count += (y2 - y1 - 1) * (x2 - x1 - 1);
                interior.push((point_count, i));
            }
        }
        let perimeter: Vec<(i32, i32)> = perimeter.into_iter().collect();
        let perimeter_count = perimeter.len() as i32;
        point_count += perimeter_count;
        Solution {
            rects,
            perimeter,
            perimeter_count,
            interior,
            point_count,
        }
    }

    fn pick(&self) -> Vec<i32> {
        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let mut random = rng.gen_range(0, self.point_count);
        if random < self.perimeter_count {
            let (x, y) = self.perimeter[(random) as usize];
            return vec![x, y];
        }
        random -= self.perimeter_count - 1;
        // binary search over interior for lowest value >= random
        let mut low = 0;
        let mut high = self.interior.len();
        while low < high {
            let mid = (low + high) / 2;
            if random > self.interior[mid].0 { low = mid + 1; }
            else { high = mid; }
        }
        let rect = &self.rects[self.interior[low].1];
        let (x1, y1, x2, y2) = (rect[0], rect[1], rect[2], rect[3]);
        let x = rng.gen_range(x1 + 1, x2);
        let y = rng.gen_range(y1 + 1, y2);
        vec![x, y]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let rects = vec![vec![1, 1, 3, 3], vec![2, 3, 4, 5]];
        let point_picker = Solution::new(rects);
        for _ in 0..50 {
            println!("{:?}", point_picker.pick());
        }
        panic!("throw panic to show results");
    }
}
