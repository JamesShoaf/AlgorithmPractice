fn main() {
    let nums: Vec<Vec<u32>> = include_str!("../input.txt")
        .lines()
        .map(|string| {
            string
                .split(',')
                .filter_map(|substring| substring.parse().ok())
                .collect()
        })
        .collect();
    let arrival: u32 = nums[0][0];
    let buses: Vec<u32> = nums.into_iter().skip(1).next().unwrap();
    println!("arrival: {}, buses: {:?}", arrival, buses);
    let (departure_time, bus) = find_first_bus_departure(arrival, &buses);
    let part_1 = (departure_time - arrival) * bus;
    println!(
        "first bus: {}, departure_time: {}, ans: {}",
        bus, departure_time, part_1
    );
    let buses_with_offset: Vec<Option<u32>> = include_str!("../input.txt")
        .lines()
        .skip(1)
        .map(|string| {
            string
                .split(',')
                .map(|substring| substring.parse().ok())
                .collect()
        })
        .next()
        .unwrap();
    let enumerated_buses = enumerate_buses(buses_with_offset);
    println!(
        "Contest_answer: {}",
        solve_shuttle_contest(enumerated_buses)
    );
}

fn find_first_bus_departure(arrival: u32, buses: &Vec<u32>) -> (u32, u32) {
    buses
        .iter()
        .map(|&bus| {
            (
                (arrival / bus) * bus + (arrival % bus != 0) as u32 * bus,
                bus,
            )
        })
        .min()
        .unwrap()
}

fn enumerate_buses(buses_with_offset: Vec<Option<u32>>) -> Vec<(u64, u64)> {
    buses_with_offset
        .into_iter()
        .enumerate()
        .filter_map(|(i, num)| {
            let num = num?;
            Some((i as u64, num as u64))
        })
        .collect()
}

fn solve_shuttle_contest(enumerated_buses: Vec<(u64, u64)>) -> u64 {
    let mut res = 0;
    let mut additive_identity = 1;
    for (offset, bus) in enumerated_buses {
        let pos_mod = (bus - offset % bus) % bus;
        while res % bus != pos_mod {
            res += additive_identity;
        }
        additive_identity *= bus;
    }
    res
}

#[test]
fn test_solve_shuttle_contest() {
    let test_vec: Vec<(u64, u64)> = vec![(0, 7), (1, 13), (4, 59), (6, 31), (7, 19)];
    assert_eq!(solve_shuttle_contest(test_vec), 1068781);
}
