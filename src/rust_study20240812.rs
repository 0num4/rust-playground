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
pub fn q1() {
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
    trait Shape {
        fn area(&self) -> i32;
    }
    struct Circle {
        r: i32,
    }
    impl Shape for Circle {
        fn area(&self) -> i32 {
            return self.r * self.r * PI as i32;
        }
    }
    struct Rectangle {
        x: i32,
        y: i32,
    }
    impl Shape for Rectangle {
        fn area(&self) -> i32 {
            return self.x * self.y;
        }
    }
    fn print_area<T>(arg: T) -> i32
    where
        T: Shape,
    {
        return arg.area();
    }
}

use std::f32::consts::PI;

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

// はい、dynキーワードとトレイトオブジェクトに関する問題を出題します。この問題は、Rustの動的ディスパッチと型消去の概念を理解するのに役立ちます。

// 問題：

// あなたは図形を扱うプログラムを作成しています。異なる種類の図形（円と正方形）を同じコレクション内で管理する必要があります。各図形は面積を計算できる必要があります。

// 以下の要件を満たすコードを実装してください：

// Shape トレイトを定義し、area() メソッドを持たせてください。このメソッドは図形の面積を計算して f64 型で返します。
// Circle 構造体と Square 構造体を定義し、それぞれ Shape トレイトを実装してください。
// Circle は半径 radius を持ちます。
// Square は一辺の長さ side を持ちます。
// Vec<Box<dyn Shape>> 型のベクターを作成し、異なる図形のインスタンスを追加してください。
// ベクター内のすべての図形の面積の合計を計算し、出力する関数 total_area を実装してください。
// メイン関数で、いくつかの Circle と Square のインスタンスを作成し、それらをベクターに追加して total_area 関数を呼び出してください。
// ボーナス課題：
// 6. 新しい図形タイプ（例：Triangle）を追加し、既存のコードを変更せずにベクターに含めて面積計算ができることを示してください。

// この問題を解いてみてください。実装が完了したら、コードを共有してください。その後、解答の解説と改善点について議論しましょう。

// Copy
// Retry

// Claude can make mistakes. Please double-check responses.

pub fn q2() {
    trait Shape {
        fn area(&self) -> f32;
    }
    struct Circle {
        radius: f32,
    }
    struct Square {
        side: f32,
    }
    impl Shape for Circle {
        fn area(&self) -> f32 {
            return self.radius * self.radius * PI;
        }
    }
    impl Shape for Square {
        fn area(&self) -> f32 {
            return self.side * self.side;
        }
    }
    let mut v: Vec<Box<dyn Shape>> = vec![];
    let c = Circle { radius: 3.0 };
    let boxedc = Box::new(c);
    let sq = Square { side: 4.0 };
    let boxedsq = Box::new(sq);
    v.push(boxedc);
    v.push(boxedsq);
    let mut aa: f32 = 0.0;
    for s in v {
        aa += s.area()
    }
    println!("{:?}", aa);
}

// fn q3() {
//     // dynに関するもっと難しい問題を出して

//     // 了解しました。より難しいdynに関する問題を出題します。この問題では、トレイトオブジェクト、ジェネリクス、クロージャ、そしてライフタイムの概念を組み合わせます。

//     // 問題：

//     // あなたは、様々な種類のデータ変換を行うライブラリを開発しています。このライブラリは、異なる型のデータに対して異なる変換を適用できる必要があります。以下の要件を満たすコードを実装してください：

//     // Transform トレイトを定義してください。このトレイトは以下のメソッドを持ちます：
//     // transform(&self, data: &str) -> String: 入力文字列を変換して新しい文字列を返します。
//     // name(&self) -> &str: 変換の名前を返します。
//     // 以下の構造体を定義し、それぞれ Transform トレイトを実装してください：
//     // UpperCase: 文字列を大文字に変換します。
//     // Reverse: 文字列を逆順にします。
//     // AddPrefix<'a>: 文字列の先頭に特定のプレフィックスを追加します。このプレフィックスはライフタイム 'a を持ちます。
//     // TransformPipeline 構造体を定義してください。この構造体は：
//     // Vec<Box<dyn Transform>> 型のフィールドを持ち、複数の変換を順番に適用できるようにします。
//     // add_transform メソッドを持ち、新しい変換を追加できるようにします。
//     // apply メソッドを持ち、すべての変換を順番に適用して結果を返します。
//     // create_custom_transform 関数を実装してください。この関数は：
//     // クロージャを引数として受け取り、そのクロージャを使用して Transform トレイトを実装する新しい型を返します。
//     // 返される型は Box<dyn Transform> です。
//     // メイン関数で：
//     // TransformPipeline のインスタンスを作成します。
//     // 事前定義された変換（UpperCase, Reverse, AddPrefix）と、create_custom_transform を使用してカスタム変換を追加します。
//     // パイプラインに文字列を適用し、結果を出力します。
//     // ボーナス課題：
//     // 6. TransformPipeline に parallel_apply メソッドを追加してください。このメソッドは rayon クレートを使用して変換を並列に適用します。

//     // この問題は、dynの高度な使用法、ジェネリクス、クロージャ、ライフタイム、そして並列処理の概念を組み合わせています。実装が完了したら、コードを共有してください。その後、解答の解説と改善点について議論しましょう。
//     trait Transform {
//         fn transform(&self, s: &str) -> String;
//         fn name(&self) -> &str;
//     }
//     struct UpperCase {}
//     impl Transform for UpperCase {
//         fn transform(&self, s: &str) -> String {
//             return s.to_uppercase().to_string();
//         }
//         fn name(&self) -> &str {
//             return "Transform";
//         }
//     }
//     struct Reverse {}
//     struct AddPrefix {}
//     struct TransformPipeline {
//         v: Vec<Box<dyn Transform>>,
//     }
//     impl TransformPipeline {
//         fn apply(&self) -> None {
//             for i in self.v {
//                 i.transform("nya")
//             }
//         }
//     }
//     let cl = |clarg| -> Box<dyn Transform> {
//         let a = clarg{

//         }
//         return ;
//     };
//     fn create_custom_transform<S, F: Transform>(s: S, f:F) -> Box<dyn Transform>{

//     }
// }
// えー↑は難しすぎるので廃棄
