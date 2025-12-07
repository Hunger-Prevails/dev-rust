use std::f64::consts;

fn main() {
    let mut data: Vec<i32> = (0..10).collect();

    data.push(10);
    data.push(11);

    let v: Vec<i32> = data.iter().map(|x| x * 2).collect();

    let sum: i32 = v.iter().sum();

    assert_eq!(sum, 132);

    println!("square sum = {}", sum);

    let x = consts::PI;

    let abs_difference = x.sin().abs();

    assert!(abs_difference < 1e-10);
}

fn square(x: f32) -> f32 {
    return x * x;
}
