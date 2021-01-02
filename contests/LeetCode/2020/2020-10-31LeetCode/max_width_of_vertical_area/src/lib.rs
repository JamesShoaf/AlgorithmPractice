/* 
Given n points on a 2D plane where points[i] = [xi, yi], Return the widest vertical area between two points such that no points are inside the area.

A vertical area is an area of fixed-width extending infinitely along the y-axis (i.e., infinite height). The widest vertical area is the one with the maximum width.

Note that points on the edge of a vertical area are not considered included in the area.
*/

pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
    let mut x_coords: Vec<i32> = points.into_iter().map(|point| point[0]).collect();
    x_coords.sort();
    x_coords.windows(2)
        .map(|slice| slice[1] - slice[0])
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
