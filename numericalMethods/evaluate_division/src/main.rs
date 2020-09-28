use evaluate_division::calc_equation;

fn main() {
    let equations = vec![
        vec![String::from("a"), String::from("b")],
        vec![String::from("e"), String::from("f")],
        vec![String::from("b"), String::from("e")],
    ];
    let values = vec![3.4, 1.4, 2.3];
    let queries = vec![
        vec![String::from("b"), String::from("a")],
        vec![String::from("a"), String::from("f")],
        vec![String::from("f"), String::from("f")],
        vec![String::from("e"), String::from("e")],
        vec![String::from("c"), String::from("c")],
        vec![String::from("a"), String::from("c")],
        vec![String::from("f"), String::from("e")],
    ];
    calc_equation(equations, values, queries);
}
