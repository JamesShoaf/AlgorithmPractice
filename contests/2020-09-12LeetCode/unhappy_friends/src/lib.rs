/* 
You are given a list of preferences for n friends, where n is always even.

For each person i, preferences[i] contains a list of friends sorted in the order of preference. In other words, a friend earlier in the list is more preferred than a friend later in the list. Friends in each list are denoted by integers from 0 to n-1.

All the friends are divided into pairs. The pairings are given in a list pairs, where pairs[i] = [xi, yi] denotes xi is paired with yi and yi is paired with xi.

However, this pairing may cause some of the friends to be unhappy. A friend x is unhappy if x is paired with y and there exists a friend u who is paired with v but:

    x prefers u over y, and
    u prefers x over v.

Return the number of unhappy friends.
*/

use std::collections::HashMap;

fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
    let mut count = 0;
    // <individual, <friend, preference rank>>
    let mut pref_map: HashMap<i32, HashMap<i32, i32>> = HashMap::new();
    for (i, prefs) in preferences.iter().enumerate() {
        let i = i as i32;
        let current = pref_map.entry(i).or_insert(HashMap::new());
        for (j, &friend) in prefs.iter().enumerate() {
            current.insert(friend, j as i32);
        }
    }
    let mut pairing_map: HashMap<i32, i32> = HashMap::new();
    for pair in pairs.iter() {
        pairing_map.insert(pair[0], pair[1]);
        pairing_map.insert(pair[1], pair[0]);
    }
    for friend in 0..n {
        let partner = *pairing_map.get(&friend).unwrap();
        for preferred in &preferences[friend as usize] {
            if *preferred == partner { break; }
            if pref_map.get(preferred).unwrap().get(&friend) <
                pref_map.get(preferred).unwrap().get(pairing_map.get(preferred).unwrap()) {
                count += 1;
                break;
            }
        }
    }
    count
}
