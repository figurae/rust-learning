pub fn zero_vec1d_flat(vec: &mut Vec<usize>, width: usize, height: usize) {
    for item in vec.iter_mut() {
        *item = 0;
    }
}

pub fn zero_vec1d_nested(vec: &mut Vec<usize>, width: usize, height: usize) {
    for i in 0..width {
        for j in 0..height {
            vec[height * i + j] = 0;
        }
    }
}

pub fn zero_vec2d(vec: &mut [Vec<usize>]) {
    for row in vec.iter_mut() {
        for item in row.iter_mut() {
            *item = 0;
        }
    }
}
