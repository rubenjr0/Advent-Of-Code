pub type Matrix = Vec<Vec<u8>>;

pub fn from_str(input: &str) -> Matrix {
    input
        .lines()
        .map(|line| line.split("").flat_map(|x| x.parse()).collect())
        .collect()
}

pub fn count_visibles(matrix: &Matrix) -> usize {
    let mut visible = 0;
    matrix.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, element)| {
            let is_visible = visible_above(&matrix, element, row_idx, col_idx)
                || visible_below(&matrix, &element, row_idx, col_idx)
                || visible_left(matrix, element, row_idx, col_idx)
                || visible_right(matrix, element, row_idx, col_idx);
            if is_visible {
                visible += 1
            }
        });
    });
    visible
}

fn above_iter<'a>(
    matrix: &'a Matrix,
    row_idx: usize,
    col_idx: usize,
) -> impl Iterator<Item = u8> + 'a {
    matrix
        .iter()
        .take(row_idx)
        .map(move |row| row[col_idx])
        .rev()
}

fn right_iter<'a>(
    matrix: &'a Matrix,
    row_idx: usize,
    col_idx: usize,
) -> impl Iterator<Item = u8> + 'a {
    matrix[row_idx].iter().skip(col_idx + 1).map(|x| *x)
}

fn below_iter<'a>(
    matrix: &'a Matrix,
    row_idx: usize,
    col_idx: usize,
) -> impl Iterator<Item = u8> + 'a {
    matrix.iter().skip(row_idx + 1).map(move |row| row[col_idx])
}

fn left_iter<'a>(
    matrix: &'a Matrix,
    row_idx: usize,
    col_idx: usize,
) -> impl Iterator<Item = u8> + 'a {
    matrix[row_idx].iter().take(col_idx).map(|x| *x).rev()
}

fn visible_above(matrix: &Matrix, element: &u8, row_idx: usize, col_idx: usize) -> bool {
    above_iter(matrix, row_idx, col_idx).all(|x| x < *element)
}

fn visible_right(matrix: &Matrix, element: &u8, row_idx: usize, col_idx: usize) -> bool {
    right_iter(matrix, row_idx, col_idx).all(|x| x < *element)
}

fn visible_below(matrix: &Matrix, element: &u8, row_idx: usize, col_idx: usize) -> bool {
    below_iter(matrix, row_idx, col_idx).all(|x| x < *element)
}

fn visible_left(matrix: &Matrix, element: &u8, row_idx: usize, col_idx: usize) -> bool {
    left_iter(matrix, row_idx, col_idx).all(|x| x < *element)
}

pub fn best_scenic_score(matrix: &Matrix) -> usize {
    let mut best_score = 0;
    matrix.iter().enumerate().for_each(|(row_idx, row)| {
        row.iter().enumerate().for_each(|(col_idx, element)| {
            let score = scenic_score(&matrix, element, row_idx, col_idx);
            if score > best_score {
                best_score = score
            }
        });
    });
    best_score
}

fn scenic_score(matrix: &Matrix, element: &u8, row_idx: usize, col_idx: usize) -> usize {
    viewing_distance_above(matrix, element, row_idx, col_idx)
        * viewing_distance_below(matrix, element, row_idx, col_idx)
        * viewing_distance_left(matrix, element, row_idx, col_idx)
        * viewing_distance_right(matrix, element, row_idx, col_idx)
}

fn viewing_distance(element: &u8, iter: impl Iterator<Item = u8>) -> usize {
    let mut visible = 0;
    for x in iter {
        visible += 1;
        if x >= *element {
            break;
        }
    }
    visible
}

fn viewing_distance_above(matrix: &Matrix, element: &u8, row_idx: usize, col_idx: usize) -> usize {
    viewing_distance(element, above_iter(matrix, row_idx, col_idx))
}

fn viewing_distance_below(matrix: &Matrix, element: &u8, row_idx: usize, col_idx: usize) -> usize {
    viewing_distance(element, below_iter(matrix, row_idx, col_idx))
}

fn viewing_distance_left(matrix: &Matrix, element: &u8, row_idx: usize, col_idx: usize) -> usize {
    viewing_distance(element, left_iter(matrix, row_idx, col_idx))
}

fn viewing_distance_right(matrix: &Matrix, element: &u8, row_idx: usize, col_idx: usize) -> usize {
    viewing_distance(element, right_iter(matrix, row_idx, col_idx))
}
