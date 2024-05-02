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

## box について

rust において全ての変数はデフォルトでスタックに確保されるが、ヒープに確保したい場合は Box を使う。
box はメモリのヒープ領域に変数を置く
box は正確にはヒープ上の T におかれたスマートポインタ
