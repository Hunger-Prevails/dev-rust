use std::f64::consts;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::env;


fn read_lines(filename: String) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    let mut lines: Vec<String> = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let trimmed = line.trim_end();
        lines.push(trimmed.to_string());
    }
    return Ok(lines);
}


fn main() {
	let filepath = env::args().nth(1).expect("please supply an argument");

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

    let read_lines_result = read_lines(filepath);

    if let Ok(names) = read_lines_result {
        println!("{:?}", names);
    }
}
