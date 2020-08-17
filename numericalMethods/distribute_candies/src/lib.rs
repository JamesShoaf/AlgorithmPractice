/* 
We distribute some number of candies, to a row of n = num_people people in the following way:

We then give 1 candy to the first person, 2 candies to the second person, and so on until we give
n candies to the last person.

Then, we go back to the start of the row, giving n + 1 candies to the first person, n + 2 candies
to the second person, and so on until we give 2 * n candies to the last person.

This process repeats (with us giving one more candy each time, and moving to the start of the row
after we reach the end) until we run out of candies.  The last person will receive all of our
remaining candies (not necessarily one more than the previous gift).

Return an array (of length num_people and sum candies) that represents the final distribution of
candies.
*/

fn distribute_candies (candies: i32, num_people: i32) -> Vec<i32> {
    let mut output = vec![0; num_people as usize];
    let mut remaining = candies;
    let mut next = 1;
    while remaining > 0 {
        output[((next - 1) % num_people) as usize] += if next < remaining {
            remaining -= next;
            next
        } else {
            let temp = remaining;
            remaining -= remaining;
            temp
        };
        next += 1;
    }
    output
}
