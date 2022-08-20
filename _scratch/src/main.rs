#![allow(dead_code, unused)]

fn main() {
    let x: Test = Test { x: 0, y: 0.0, z: TestEnum::Hello };
    let y: Test = Test { x: 0, y: 0.0, z: TestEnum::Hello };
    let z: Test = Test { x: 0, y: 0.0, z: TestEnum::Hello };

    let struct_vec = vec![x, y, z];
    let int_vec = vec![1, 2, 3];

    let new_int_vec: Vec<i32> = int_vec.into_iter().map(|x| x + 1).collect();
    let new_struct_vec: Vec<Test> = struct_vec.into_iter().map(|test| test.increase_x()).collect();
    println!("{:?}", new_struct_vec);
    println!("{:?}", new_int_vec);
}

#[derive(Debug)]
struct Test {
    x: u32,
    y: f32,
    z: TestEnum,
}

#[derive(Debug, Clone, Copy)]
enum TestEnum {
    Hello,
    World,
}

impl Test {
    fn increase_x(&self) -> Self {
        Self {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        }
    }
}
