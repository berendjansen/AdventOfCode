use std::fs::File;
use ndarray::{Array2, Axis};
use std::io::Read;

pub fn get_file_contents(name: &str) -> std::io::Result<Vec<String>> {
    let mut buffer = String::new();
    let mut file = File::open(name)?;

    file.read_to_string(&mut buffer).unwrap();

    Ok(buffer.trim().split("\n").map(|x| String::from(x)).collect())
}

pub trait Solver {
    fn part1(&self, input: &[&str]) -> String;
    fn part2(&self, input: &[&str]) -> String;
}

pub fn matrix_from_input(input: &[&str]) -> Array2<char> {
    let vec2d = input
        .iter()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();

    matrix_from_nested_vec(vec2d)
}

pub fn matrix_from_nested_vec<T: Default + Copy>(v: Vec<Vec<T>>) -> Array2<T> {
    let mut arr = Array2::<T>::default((v.len(), v[0].len()));
    for (i, mut r) in arr.axis_iter_mut(Axis(0)).enumerate() {
        for (j, c) in r.iter_mut().enumerate() {
            *c = v[i][j];
        }
    }
    arr
}
