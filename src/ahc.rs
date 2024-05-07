// https://atcoder.jp/contests/ahc002/tasks/ahc002_a

// use std::fmt::Debug;

use std::io::Stdin;

use rand::{thread_rng, Rng};
// // fn p<T: Debug>(v: T) {
// //     println!("{:#?}", v);
// // }
// macro_rules! pp {
//     ($($arg:expr),*) => {
//         $(
//             println!("{:?}", $arg);
//         )*
//     }
// }

#[derive(Debug)]
struct tile {
    walked: bool,
    tile_size_x: Vec<i8>, // 1~2のvec。中身はマスの得点
    tile_size_y: Vec<i8>, // 1~2のvec。中身はマスの得点
}

#[derive(Debug)]
struct Grid {
    x: i32,
    y: i32,
    world: Vec<Vec<i8>>, // 得点は0~99
}
impl Grid {
    fn new(x: i32, y: i32) -> Grid {
        let mut t = thread_rng();
        // let world = vec![vec![t.gen_range(0..100); x as usize]; y as usize];
        let mut v: Vec<Vec<i8>> = Vec::new();
        for y in 0..y {
            let mut v_x: Vec<i8> = Vec::new();
            for x in 0..x {
                v_x.push(t.gen_range(0..100))
            }
            v.push(v_x);
        }
        return Grid { x, y, world: v };
    }
    fn up(&mut self) {
        self.y -= 1;
    }
    fn down(&mut self) {
        self.y += 1;
    }
    fn left(&mut self) {
        self.x -= 1;
    }
    fn right(&mut self) {
        self.x += 1;
    }
    fn display(&mut self) {
        for i in &self.world {
            println!("{:?}", i);
        }
    }
}

pub fn main() {
    // read_input();
    let mut g = Grid::new(50, 50);
    let init_x = rand::thread_rng().gen_range(0..50);
    let init_y = rand::thread_rng().gen_range(0..50);
    g.x = init_x;
    g.y = init_y;
    println!("{:?}", g);
    log::warn!("nyaaa");
}

pub fn read_input() {
    // input! {
    //     n: i8,
    //     m: i8
    // }
    println!("入力して");
    let mut s = String::new();
    loop {
        std::io::stdin().read_line(&mut s).unwrap();
        println!("入力した文字列 {}", s);
    }
}
