pub mod part_1 {
    use std::collections::HashMap;

    pub fn sum_values(lines: &Vec<Vec<&str>>) -> u64 {
        let mut map: HashMap<usize, u64> = HashMap::new();
        let mut mask: Vec<(usize, u64)> = Vec::new();
        for line in lines.iter() {
            if line[0] == "mask" {
                mask = parse_mask(line[1]);
            } else {
                let address = parse_address(line[0]);
                let value = mask_value(line[1].parse().unwrap(), &mask);
                map.insert(address, value);
            }
        }
        map.values().sum()
    }

    fn mask_value(mut val: u64, mask: &Vec<(usize, u64)>) -> u64 {
        for &(i, bit) in mask.iter() {
            let shift = 1 << i;
            if bit == 1 {
                val |= shift;
            } else {
                val &= !shift;
            }
        }
        val
    }

    // 1X000X0101XX101101X01X101X1000111X00
    fn parse_mask(mask: &str) -> Vec<(usize, u64)> {
        let mut chars: Vec<char> = mask.chars().collect();
        chars.reverse();
        chars
            .into_iter()
            .enumerate()
            .filter_map(|(i, c)| {
                let num = c.to_digit(10)?;
                Some((i, num as u64))
            })
            .collect()
    }

    // mem[5265]
    fn parse_address(address: &str) -> usize {
        address
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap()
    }
    mod tests {
        #[test]
        fn test_parse_mask() {
            let test_tuples = vec![("XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X", vec![(1, 0), (6, 1)])];
            for (mask, expected) in test_tuples {
                assert_eq!(super::parse_mask(mask), expected);
            }
        }
        #[test]
        fn test_mask_val() {
            let test_tuples = vec![(
                "XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X",
                vec![(11, 73), (101, 101), (0, 64)],
            )];
            for (mask, test_vals) in test_tuples {
                let mask = super::parse_mask(mask);
                for (val, expected) in test_vals {
                    assert_eq!(super::mask_value(val, &mask), expected);
                }
            }
        }
    }
}

pub mod part_2 {
    const MASK_LEN: usize = 36;
    use std::collections::HashMap;

    pub fn sum_values(lines: &Vec<Vec<&str>>) -> u64 {
        let mut map: HashMap<u64, u64> = HashMap::new();
        let mut mask = Vec::new();
        for line in lines.iter() {
            if line[0] == "mask" {
                mask = line[1].chars().collect();
            } else {
                masked_write(
                    parse_address(line[0]),
                    line[1].parse().unwrap(),
                    0,
                    &mask,
                    &mut map,
                );
            }
        }
        map.values().sum()
    }

    fn masked_write(
        add: u64,
        val: u64,
        index: usize,
        mask: &Vec<char>,
        map: &mut HashMap<u64, u64>,
    ) {
        if index == MASK_LEN {
            map.insert(add, val);
            return;
        }
        let shift = MASK_LEN - 1 - index;
        if let Some(digit) = mask[index].to_digit(2) {
            return masked_write(add | (digit as u64) << shift, val, index + 1, mask, map);
        }
        masked_write(add | (1 << shift), val, index + 1, mask, map);
        masked_write(add & !(1 << shift), val, index + 1, mask, map);
    }

    fn parse_address(address: &str) -> u64 {
        address
            .chars()
            .filter(|c| c.is_digit(10))
            .collect::<String>()
            .parse()
            .unwrap()
    }
}
