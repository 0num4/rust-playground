# rust-playground

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
