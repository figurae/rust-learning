#![allow(dead_code, unused)]
use ndarray::Array2;

fn main() {
    let mut free_z = Array2::from_elem((10, 10), TestEnum::Hello);
    for z in &mut free_z {
        *z = TestEnum::World;   
    }

    dbg!(&free_z);
}

#[derive(Debug)]
struct Test {
    x: usize,
    y: usize,
    z: Array2<TestEnum>,
}

impl Test {
    fn new(z: Array2<TestEnum>) -> Self {
        Test {
            x: 0,
            y: 0,
            z
        }
    }
}

#[derive(Clone, Debug)]
enum TestEnum {
    Hello,
    World,
}
