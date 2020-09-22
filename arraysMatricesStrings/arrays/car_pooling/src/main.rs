use car_pooling::car_pooling;

fn main() {
    let test = vec![
        vec![2, 1, 5],
        vec![3, 3, 7],
    ];
    let capacity = 4;
    car_pooling(test, capacity);
}