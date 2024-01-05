# rust のビルドオプション周り

cargo build
リリースビルドとターゲットを指定して現在のディレクトリにあるプロジェクトをビルドする

curgo run hoge fuga と言う引数を渡して実行する

cargo fmt・・・フォーマットする。go fmt っぽい

curgo rustc 特殊なコマンド。rustc コンパイラ を直接実行する。cargo build とかではなく生の rustc に引数を渡したい時とか。

cargo build を打つと target/いかにいろんなファイルができる。

```
rust-playground on  master [!?] is 📦 v0.1.0 via 🦀 v1.75.0 on ☁️  r.oonuma@matsuri-tech.com took 19s
❯ tree target
target
├── CACHEDIR.TAG
└── debug
    ├── build
    ├── deps
    │   ├── librust_playground-2fd183ff82cff564.rmeta
    │   ├── librust_playground-cc9cf5698ec436d3.rmeta
    │   ├── rust_playground-2fd183ff82cff564.d
    │   ├── rust_playground-cc9cf5698ec436d3.d
    │   ├── rust_playground-feb6bf489fb6d137
    │   ├── rust_playground-feb6bf489fb6d137.1f714csgb8bbtvfs.rcgu.o
    │   ├── rust_playground-feb6bf489fb6d137.20n8nmgpw7iytwqs.rcgu.o
    │   ├── rust_playground-feb6bf489fb6d137.23pyb94gbxunwmkg.rcgu.o
    │   ├── rust_playground-feb6bf489fb6d137.2a7pr9x3v9z3wdbp.rcgu.o
    │   ├── rust_playground-feb6bf489fb6d137.2ab6ey7poben4y1j.rcgu.o
    │   ├── rust_playground-feb6bf489fb6d137.3kpw2px6c50e0sb3.rcgu.o
    │   ├── rust_playground-feb6bf489fb6d137.4rtzke2i2v4ew8yh.rcgu.o
    │   └── rust_playground-feb6bf489fb6d137.d
    ├── examples
    ├── incremental
    │   ├── rust_playground-2jjs5l3rq8fbv
    │   │   ├── s-gs6ol9qclg-19pnkmn-ah7mjyu9l0fbk4x0nknh60fq1
    │   │   │   ├── dep-graph.bin
    │   │   │   ├── query-cache.bin
    │   │   │   └── work-products.bin
    │   │   └── s-gs6ol9qclg-19pnkmn.lock
    │   ├── rust_playground-2zov1u0s7d9h2
    │   │   ├── s-gs6ol9qcl0-jvsu59-50uxmrjkljneyu9hv8ixmcd5d
    │   │   │   ├── dep-graph.bin
    │   │   │   ├── query-cache.bin
    │   │   │   └── work-products.bin
    │   │   └── s-gs6ol9qcl0-jvsu59.lock
    │   └── rust_playground-mdktvrw43vl3
    │       ├── s-gs6p3e2tm9-10piuvn-1yxfj2g4gyz9vb66hymj0ybh0
    │       │   ├── 1f714csgb8bbtvfs.o
    │       │   ├── 20n8nmgpw7iytwqs.o
    │       │   ├── 23pyb94gbxunwmkg.o
    │       │   ├── 2a7pr9x3v9z3wdbp.o
    │       │   ├── 2ab6ey7poben4y1j.o
    │       │   ├── 3kpw2px6c50e0sb3.o
    │       │   ├── 4rtzke2i2v4ew8yh.o
    │       │   ├── dep-graph.bin
    │       │   ├── query-cache.bin
    │       │   └── work-products.bin
    │       └── s-gs6p3e2tm9-10piuvn.lock
    ├── rust-playground
    └── rust-playground.d

12 directories, 35 files

```

```
❯ ./target/debug/rust-playground
Hello, world!
ガチャガチャ
```
