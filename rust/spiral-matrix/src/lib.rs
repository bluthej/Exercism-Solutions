pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut matrix = vec![vec![0; size as usize]; size as usize];
    let (mut i, mut j): (i32, i32) = (0, -1);
    let (mut di, mut dj): (i32, i32) = (0, 1);
    let mut n = 0;
    let mut upper_limit = size;
    while n < size * size {
        for _ in 0..upper_limit {
            i += di;
            j += dj;
            n += 1;
            matrix[i as usize][j as usize] = n;
        }
        if dj.abs() == 1 {
            upper_limit -= 1;
        }
        (di, dj) = (dj, -di);
    }
    matrix
}
