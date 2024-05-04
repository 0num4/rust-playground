はい、もう少し難しい問題を出題しますね。今度は、文字列処理に関する問題です。

問題：
文字列のベクター（`Vec<String>`）が与えられたとき、各文字列の最初の文字を大文字に変換し、残りの文字を小文字に変換する関数 `capitalize_first_letter` を実装してください。変換後の文字列のベクターを返します。

関数のシグネチャは以下の通りです：

```rust
fn capitalize_first_letter(strings: Vec<String>) -> Vec<String> {
    // ここに実装を記述
}
```

例：

```rust
let strings = vec![String::from("hello"), String::from("WORLD"), String::from("rust")];
let capitalized = capitalize_first_letter(strings);
println!("{:?}", capitalized);
// 出力：["Hello", "World", "Rust"]
```

この問題では、以下のような Rust の概念を活用する必要があります：

1. ベクターの反復処理
2. 文字列の操作（先頭文字の取得、大文字・小文字の変換）
3. 新しい文字列の作成
4. 新しいベクターの作成と返却

問題を解く際は、可読性とパフォーマンスの両方を考慮してください。また、必要に応じて Rust のドキュメントを参照し、適切なメソッドを探すことをお勧めします。

回答を考えてみて、実装が完了したら教えてください。一緒に解説していきましょう。テストコードも忘れずに書くようにしてくださいね。
