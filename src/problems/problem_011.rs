use std::cmp;

const GRID_STR: &str = "
08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48";

pub fn solve() {
    let (x_limit, y_limit) = calc_shape(GRID_STR);
    let grid = grid_to_vec(GRID_STR);

    // get all horizontal sums:
    let prod_x: Vec<u64> = (0..grid.len()).into_iter()
        .map(|i| calc_x_and_y_from_index(i, x_limit, y_limit))
        .map(|(x, y)| get_prod_x(&grid, x, y, x_limit))
        .map(|e| e.unwrap_or(0))
        .collect();

    let prod_y: Vec<u64> = (0..grid.len()).into_iter()
        .map(|i| calc_x_and_y_from_index(i, x_limit, y_limit))
        .map(|(x, y)| get_prod_y(&grid, x, y, x_limit, y_limit))
        .map(|e| e.unwrap_or(0))
        .collect();

    let prod_diag_up: Vec<u64> = (0..grid.len()).into_iter()
        .map(|i| calc_x_and_y_from_index(i, x_limit, y_limit))
        .map(|(x, y)| get_prod_diag_up(&grid, x, y, x_limit, y_limit))
        .map(|e| e.unwrap_or(0))
        .collect();
    
    let prod_diag_down: Vec<u64> = (0..grid.len()).into_iter()
        .map(|i| calc_x_and_y_from_index(i, x_limit, y_limit))
        .map(|(x, y)| get_prod_diag_down(&grid, x, y, x_limit, y_limit))
        .map(|e| e.unwrap_or(0))
        .collect();

    let prod: Vec<u64> = prod_x.into_iter()
        .chain(prod_y)
        .chain(prod_diag_up)
        .chain(prod_diag_down).collect();
    let max = prod.into_iter().max().unwrap();

    println!("Solution to problem 11:");
    println!("{:?}", max);
}

fn grid_to_vec(grid_str: &str) -> Vec<u64> {
    let grid: Vec<u64> = grid_str.split_whitespace()
        .map(|e| e.parse::<u64>().unwrap()).collect();
    grid
}

fn calc_x_and_y_from_index(i: usize, x_limit: usize, y_limit: usize) -> (usize, usize) {
    let x = i % x_limit;
    let y = i / y_limit;
    (x, y)
}

fn calc_shape(grid_str: &str) -> (usize, usize) {
    let mut y: usize = 0;
    let mut x: usize = 0;

    let rows: Vec<&str> = grid_str.split('\n').collect();
    for row in rows {
        let _x = row.split(' ').count();
        x = cmp::max(x, _x);

        if _x > 1 {
            y += 1;
        }
    }

    let x: usize = x;
    let y: usize = y;
    (x, y)
}

fn get_prod_x(grid: &Vec<u64>, x: usize, y: usize, x_limit: usize) -> Option<u64> {
    if x + 4 > x_limit {
        return None
    } else {
        // let x = x + 4;
        let i = x_limit * y + x;
        Some(grid[i..(i+4)].into_iter().product())
    }
}

fn get_prod_y(grid: &Vec<u64>, x: usize, y: usize, x_limit: usize, y_limit: usize) -> Option<u64> {
    if y + 4 > y_limit {
        None
    } else {
        let elements: Vec<Option<u64>> = vec![
            get_elem(grid, x, y, x_limit, y_limit),
            get_elem(grid, x, y+1, x_limit, y_limit),
            get_elem(grid, x, y+2, x_limit, y_limit),
            get_elem(grid, x, y+3, x_limit, y_limit),
        ];

        Some(elements.iter().map(|e| e.unwrap_or(0)).product())
    }
}

fn get_prod_diag_up(grid: &Vec<u64>, x: usize, y: usize, x_limit: usize, y_limit: usize) -> Option<u64> {
    if x + 4 > x_limit || y < 4 {
        None
    } else {
        let elements: Vec<Option<u64>> = vec![
            get_elem(grid, x, y, x_limit, y_limit),
            get_elem(grid, x+1, y - 1, x_limit, y_limit),
            get_elem(grid, x+2, y - 2, x_limit, y_limit),
            get_elem(grid, x+3, y - 3, x_limit, y_limit)
        ];

        if elements.iter().any(|e| e.is_none()) {
            None
        } else {
            Some(elements.iter().map(|e| e.unwrap()).product())
        }
    }
}

fn get_prod_diag_down(grid: &Vec<u64>, x: usize, y: usize, x_limit: usize, y_limit: usize) -> Option<u64> {
    if x + 4 > x_limit || y + 4 >= y_limit {
        None
    } else {
        let elements: Vec<Option<u64>> = vec![
            get_elem(grid, x, y, x_limit, y_limit),
            get_elem(grid, x+1, y + 1, x_limit, y_limit),
            get_elem(grid, x+2, y + 2, x_limit, y_limit),
            get_elem(grid, x+3, y + 3, x_limit, y_limit)
        ];

        if elements.iter().any(|e| e.is_none()) {
            None
        } else {
            Some(elements.iter().map(|e| e.unwrap()).product())
        }
    }
}

fn get_elem(grid: &Vec<u64>, x: usize, y: usize, x_limit: usize, y_limit: usize) -> Option<u64> {
    if x >= x_limit {
        None
    } else if y >= y_limit {
        None
    } else {
        Some(grid[y * x_limit + x])
    }
}
