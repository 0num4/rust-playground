// rustの練習をしたいです。whereとimplとstructを使って難しめの問題を出して
// ChatGPT
// メモリを更新しました
// 次のようなRustの問題に挑戦してみてください。

// 問題：
// あるデータ構造 Shape があり、それを実装する異なる形状（例えば、Circle や Rectangle）があるとします。これらの形状はそれぞれの面積を計算するメソッド area を持っています。

// この問題では、ジェネリック型とトレイト境界 (where) を使用して、複数の形状に対して共通のインターフェースを定義し、それぞれの形状の面積を計算するメソッドを実装してください。

// 要件：
// 構造体 Circle と Rectangle を定義し、それぞれに必要なフィールドを含める。
// それぞれの構造体に impl ブロックを使って area メソッドを実装する。
// トレイト Shape を定義し、このトレイトに area メソッドのシグネチャを含める。
// where を使ってジェネリックな関数 print_area を実装し、Shape を実装している型のみを受け入れる。この関数は受け取った形状の面積を出力する。
// 制約：
// where 節を用いて、ジェネリック関数のトレイト境界を指定してください。
// 各形状の area メソッドは正確な面積を返すように実装してください。

// 魔法の英文: impl Trait for Struct

// trait Shape {
//     fn area(&self) -> i32;
// }

// struct Rectangle {
//     size: i32,
// }

// impl Rectangle for Shape {}

// // impl Shape for Rectangle {
// //     fn area(&self) {
// //         3;
// //     }
// // }

// fn print_area<T>()
// where
//     T: Shape,
// {
// }

// pub fn main() {
//     print_area();
// }
trait Perimeter {
    fn perimeter(&self) -> i32;
}
struct Rectangle {
    width: i32,
    height: i32,
}

impl Perimeter for Rectangle {
    // これはできない。Perimeterにはperimeterメソッドしか許容してないので。
    // fn size(&self) -> i32 {
    //     self.width * self.height
    // }
    fn perimeter(&self) -> i32 {
        return self.width * self.height;
    }
}

impl Rectangle {
    //↑をやりたい場合はこうやって分割する。そうすると下のmainの中で認識できる。
    fn size(&self) -> i32 {
        self.width * self.height
    }
}

pub fn main() {
    let s = Rectangle {
        width: 3,
        height: 4,
    };
    let a = s.size();
}
