use std::time::Instant;

use crate::matrix::best_scenic_score;

mod matrix;

fn main() {
    let input = include_str!("../input.txt");
    let matrix_parsing = Instant::now();
    let matrix = matrix::from_str(input);
    let matrix_parsing_duration = matrix_parsing.elapsed();
    println!("Matrix parsing took {matrix_parsing_duration:?}");

    let part_one_time = Instant::now();
    let visible = matrix::count_visibles(&matrix);
    let part_one_time = part_one_time.elapsed();
    println!("Part one: {visible} trees visible from outside the grid ({part_one_time:?})");

    let part_two_time = Instant::now();
    let best_scenic_score = best_scenic_score(&matrix);
    let part_two_time = part_two_time.elapsed();
    println!("Part two: {best_scenic_score} is the best scenic score ({part_two_time:?})");
}

#[cfg(test)]
mod tests {
    use crate::matrix::{best_scenic_score, count_visibles, from_str};

    #[test]
    fn part_one() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let matrix = from_str(input);
        let visible = count_visibles(&matrix);
        assert_eq!(visible, 21);
    }

    #[test]
    fn part_two() {
        let input = "30373\n25512\n65332\n33549\n35390";
        let matrix = from_str(input);
        let best_scenic_score = best_scenic_score(&matrix);
        assert_eq!(best_scenic_score, 8);
    }
}
