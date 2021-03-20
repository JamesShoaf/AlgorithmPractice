/*
There is a school that has classes of students and each class will be having a final exam. You are
given a 2D integer array classes, where classes[i] = [passi, totali]. You know beforehand that in the
ith class, there are totali total students, but only passi number of students will pass the exam.

You are also given an integer extraStudents. There are another extraStudents brilliant students that
are guaranteed to pass the exam of any class they are assigned to. You want to assign each of the
extraStudents students to a class in a way that maximizes the average pass ratio across all the classes.

The pass ratio of a class is equal to the number of students of the class that will pass the exam
divided by the total number of students of the class. The average pass ratio is the sum of pass ratios
of all the classes divided by the number of the classes.

Return the maximum possible average pass ratio after assigning the extraStudents students. Answers
within 10-5 of the actual answer will be accepted.
*/

use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(PartialOrd, PartialEq)]
struct OrdF64(f64);

impl Eq for OrdF64 {}

impl Ord for OrdF64 {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.0 - other.0 > 0.0 {
            return Ordering::Greater;
        }
        if self.0 - other.0 < 0.0 {
            return Ordering::Less;
        }
        Ordering::Equal
    }
}

fn difference(class: &Vec<i32>) -> f64 {
    let current: f64 = class[0] as f64 / class[1] as f64;
    let plus_one: f64 = (class[0] + 1) as f64 / (class[1] + 1) as f64;
    plus_one - current
}

pub fn max_average_ratio(mut classes: Vec<Vec<i32>>, mut extra_students: i32) -> f64 {
    let mut heap: BinaryHeap<(OrdF64, usize)> = BinaryHeap::new();
    for (i, class) in classes.iter().enumerate() {
        heap.push((OrdF64(difference(class)), i));
    }
    while extra_students > 0 {
        if let Some((_, i)) = heap.pop() {
            classes[i][0] += 1;
            classes[i][1] += 1;
            heap.push((OrdF64(difference(&classes[i])), i));
        }
        extra_students -= 1;
    }
    let class_count = classes.len() as f64;
    classes
        .into_iter()
        .map(|class| class[0] as f64 / class[1] as f64)
        .sum::<f64>()
        / class_count
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
