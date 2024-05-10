# rust-playground

[![rust-lint](https://github.com/0num4/rust-playground/actions/workflows/lint.yml/badge.svg)](https://github.com/0num4/rust-playground/actions/workflows/lint.yml)

```
cuargo run
```

## jansoul-gacha

## 各プロダクトの調査

### cleaning

```
cargo rustc --bin xxx -- -Dwarnings -Adead_code -Adeprecated && cargo fmt -- --check
```

が ci 上で走ってる

### mjai.app

```
https://github.com/smly/mjai.app/blob/fb34c4837a25c61e0cf17b6cfa6253611cdb5cb0/.github/workflows/cargo_test.yml#L26
```

cirgo fmt、clippy、test が走ってる

## tokio

**前提として rust の main は非同期にはできない。** #[tokio::main]をつけると、main 関数が非同期関数になる。

## await 演算子

rust の async/await は、Future を返す
Future に await をつけると、その Future が完了するまで待つ

## box について

https://zenn.dev/torohash/articles/5264df373d50af

rust において全ての変数はデフォルトでスタックに確保されるが、ヒープに確保したい場合は Box を使う。
box はメモリのヒープ領域に変数を置く
box は正確にはヒープ上の T におかれたスマートポインタ

## Result<(), \_>

\_の部分は任意のエラー型を返す

## dyn

dyn は trait を作成する時に使われる

## tokio2

tcplistener で listen 出来て、tcpstream で get できる

tcplistener や tcpstream は std::net::tcplistener と tokio::net::tcpstream がある。両者は Future を返すか返さないかの違いがあり.await が使えるか変わるので注意

## rust3

loop{}← これで無限ループが作れる

## tokio::spawn()

tokio runtime 上で新しいスレッドを作るための関数。
tokio::spawn は、Tokio ランタイム上で新しい非同期タスクを生成するための関数です。spawn 関数は、非同期の処理を別のタスクとして実行することを可能にし、メインタスクとは独立して並行に実行されます。

## async move {}

move キーワードは所有権を move する。
別のスレッドを作るときなどは move した方が良い

```
以下は、moveキーワードを使用しない場合の例です。
rustCopy codeuse std::sync::Arc;
use tokio;

#[tokio::main]
async fn main() {
    let data = Arc::new(vec![1, 2, 3, 4, 5]);

    let handle = tokio::spawn(async {
        // dataへの参照を使用する
        let sum = data.iter().sum::<i32>();
        println!("Sum: {}", sum);
    });

    // dataを変更する
    if let Some(value) = Arc::get_mut(&mut data) {
        value.push(6);
    }

    handle.await.unwrap();
}
```

# Q&A formatter が効かない

**[rust]セクションの "editor.defaultFormatter": "rust-lang.rust-analyzer"が一番重要**

.vscode/settings.json

```
{
  "editor.formatOnSave": true,
  "[rust]": {
    "editor.formatOnSave": true,
    "editor.defaultFormatter": "rust-lang.rust-analyzer"
  }
}

```

# use hyper_util::rt::TokioIo;が無い

cargo add tokio-util だけじゃだめ。
feature = ["full"]を追加しないと使えない

```
cargo add tokio-util --features full
```

# vec の操作

iter() の操作(検索)には find, filter,retain などがある。

**vec をそのまま検索したいときは get メソッドが使える！！**

iter()の内容を変えたいときは iter_mut()を使う

# Result の基礎

Ok(val)で Result が作れる
![alt text](image.png)
?は剥がすのではなく伝搬させる
https://claude.ai/chat/0778e660-3c79-46fb-b1a4-b4757c03a484

# file::open

src/以下ではなくプロジェクト root をみる

# rust の test

`#[cfg(test)]`をつけるとコンパイル時に無視される。
rust test を実行したときしか`#[cfg(test)]`の内容はコンパイルされない
use super::\*;で親モジュールの関数を使える。test の場合は mod tests で{}スコープを分けるから親の関数を use super::\*;で使うことができる

test は pub を付ける必要がない。

# map と collect の謎

これは map の中が実行されない:thinking_face:

```
pub fn capitalize_first_letter(vecStr: Vec<String>) -> Vec<String> {
    let vecStr2 = vec!["a", "ff", "dddd"];
    let m = vecStr.iter().map(|x| {
        println!("aaa {:?}", x);
        x.to_string();
    });
    print!("{:?}", m);

    return Vec::new();
}

```

collect()を後ろにつけてこうすると map の中が実行された

```
pub fn capitalize_first_letter(vecStr: Vec<String>) -> Vec<String> {
    let vecStr2 = vec!["a", "ff", "dddd"];
    let m: Vec<_> = vecStr
        .iter()
        .map(|x| {
            println!("aaa {:?}", x);
            x.to_string();
        })
        .collect();
    print!("{:?}", m);

    return Vec::new();
}

```

map メソッドでは遅延評価がされるまで要素にふれるまで実行されない

こういうのは for に置き換えたほうがいいかも

# collect の型推論

`collect::<Vec<_>>()`のように collect は絶対型推論が必要

なぜなら collect は最終的な型の形がそこの推論に依存するので、つまり願えばなんでもできる魔法のメソッド

## collect 続き

https://qiita.com/hadashiA/items/d0c34a4ba74564337d2f
collect()は FromIterator<T>という trait の関数。

# hashMap もヤバくね？

上に関連してなんですが HashMap::new()も宣言した型に明示的に型をつけないといけない。普通に new()の引数で関数わたしたり HashMap::<i32,i32>new()みたいな感じじゃないんだ…

# &char も char も同じメソッドが使える

```
&charに対してもcharに対してもis_ascii_alphabetic()が使えるという認識でいいですか？はい、その認識で正しいです。is_ascii_alphabetic()メソッドは、char型の値に対して直接使用することも、&char型の参照に対して使用することもできます。
char型は、コピー可能な型（Copy traitを実装している型）の一つです。コピー可能な型は、値がコピーされるため、所有権の移動が発生せず、参照を取り外す必要がありません。
したがって、以下のように、char型の値に対しても、&char型の参照に対しても、is_ascii_alphabetic()メソッドを直接使用できます。
```

大体理解したので一旦終わり！

# rust doc

rust-analyzer 本当にキモくて///にすると doc として認識してくれる。
cargo doc でドキュメントを生成できる
生成されたものは target/doc 以下に html として生成される。
![alt text](image-1.png)

# rust の Arc や Rc について

https://qiita.com/qnighy/items/4bbbb20e71cf4ae527b9
Arc は参照カウントを使ったスマートポインタであり、データや状態を共有できる
Box の亜種っぽさを感じる。
Arc::new(42)で作成できる。ただしこの状態だと参照を共有してるので書き込めなくて、

# mutex について

Mutex::new()は基本的に display メソッドを持たない
lock()して unwrap()すると初めて中身が見える

mutex は可変性コンテナとも呼ばれる

```
ミューテックスは、2つの規則を覚えておく必要があるため、難しいという評判があります:

データを使用する前にロックの獲得を試みなければならない。
ミューテックスが死守しているデータの使用が終わったら、他のスレッドがロックを獲得できるように、 データをアンロックしなければならない。

```

Mutex から lock()をして unwrap()すると MutexGuard という型が返ってくる

![alt text](image-2.png)

lock()を取ってからもう一回 lock()を取ろうとするとずっと止まる。１つ目の lock が開放されないから

これはだめ(ずっと止まった状態になる)
![alt text](image-4.png)
これは OK
![alt text](image-3.png)

MutexGuard 型が出てきたら基本的に\*を付けて参照先を見れば ok

# threads について

thread::spawn で threads をつくる。
spawn の返り値は JoinHandle 型

std::thread を async にすることは出来ないので、tokio::spawn を使う
これはできない

```
pub fn q4_pre() {
    for i in 1..10 {
        let t = thread::spawn(|| {
            println!("5s待ちます");
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await?;
            println!("5s待ちました");
        });
        let t_res = t.join().unwrap();
        println!("{:?}", t_res)
    }
}

```

tokio::spawn なら join()は必要なくて、t.await で良い

# channel と mpsc について

# drop()

プログラムが終了すると勝手に呼ばれる。
明示的に呼ぶこともできる。

# error

```
use of moved value: `shared_int`
value moved into closure here, in previous iteration of looprustcClick for full compiler diagnostic
q4.rs(37, 41): use occurs due to use in closure
q4.rs(33, 9): move occurs because `shared_int` has type `Arc<std::sync::Mutex<i32>>`, which does not implement the `Copy` trait
```

とりあえず色々悩まされたけど解決した、もう少し詳しくなったら書く

# File::read と BufReader::read の違い

bufread のほうが早い、前者は 4byte ずつ読むので syscall を大量に呼ぶ

# seek

BufReader に seek メソッドが生えている。

# &mut の意味

可変参照。
通常の参照は参照先の値は変更できないが、&mut は変更できる
可変参照の String と普通の string は違う

# f.try_clone()

File::try_clone()はファイル構造体のために作られた特別なあれ。

# HashMap の entory

https://qiita.com/hystcs/items/75183bcf38bf95cc2ce0

神の説明過ぎる
![alt text](image-5.png)
mutex は send や sync は実装していないので包む必要がある

# スマートポインタについて

ポインタにはいくつか種類がある

- 参照・・・参照は&で表される。参照は所有権を持たず、所有権を借用するだけである。
- スマートポインタ、スマポ

# rust で乱数生成

random クレートを入れる必要がある
https://sehermitage.web.fc2.com/devel/rust_rand.html
thread_rng()で疑似乱数生成器を作って
r.gen()で乱数を生成する

# rust の print について

https://chat.openai.com/share/3699e7f8-8f6c-46d3-8245-bdc4eca348b6
"{}"が使えるのは**ディスプレイ trait を実装している型のみ**
それ以外は"{:?}"を使う。"{:#?}"できれいに表示できる

全ての型は debug trait を実装している

# ジェネリクスの型みたいなやつ

T は debugtrait を実装している必要がある Generic 型

```rust
fn p<T: Debug>(v: T){

}
```

# rust のベクター

vec はヒープ領域に確保される

# macro

macro_rules で実装できる。マッチ式っぽい

```rust
macro_rules! foo {
    () => {};
}

```

他のファイルで使うには#[macro_export]が必要

macro 内では return は使えない！！返り値とかは any になってるので自由

```
variable 'x' is still repeating at this depthrustcClick for full compiler diagnostic
```

```
これで繰り返しを表現できる
$($elem:expr),*
```

```
macro_rules! is_vec_u64 {
    ($expr:expr) => {
        match $expr {
            vec![$($elem:expr),*] => {
                true $(&& { let _ = $elem as u64; true })*
            }
            _ => false,
        }
    };
}
```

# String と&str と'static str の違い

&str は一般的な文字列スライスで任意のライフタイムを持つ。参照が有効な限り続く。
&'static str はプログラムが終了するまで有効

# ライフタイムとライフタイム境界

A<T: Debug>は trait 境界
A<T: 'static>はライフタイム境界

```

```

```
consider using the `'a` lifetime: `'a `rustcE0106

fn longest<'a>(x: &'a str, y: &'a str) -> &str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}
```

# ライフタイム 2

「より長い」を表現するには<>の中でライフタイムの宣言みたいなことをする必要がある。
<'a, 'b: 'a>のように

全ての変数にはライフタイム宣言が暗示的に隠れている。

```
cannot return value referencing local data `person.name`
returns a value referencing data owned by the current function
```

# split 構造体

String に対して split すると Split 構造体が返ってくる

# rust で datetime の parse

chrono を使う
