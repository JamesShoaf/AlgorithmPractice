/*
You are given coordinates, a string that represents the coordinates of a square of the chessboard. Below is a chessboard for your reference.
*/

pub fn square_is_white(coordinates: String) -> bool {
    let chars: Vec<char> = coordinates.chars().collect();
    let col = chars[0] as u8 - 'a' as u8;
    let row = chars[1].to_digit(10).unwrap();
    (col as u32 + row) % 2 != 1
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
