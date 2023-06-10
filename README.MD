# Rust The book 写経メモ

なんとなく分からなかったところをメモしていきます。

## 型

- int 型<br>
  型サイズの bit 数を $n=\set{8, 16, 32, 64, 128, i}$ とする (i はコンピュータアーキテクチャに依存、32bit なら 32、64bit なら 64)
  - signed int 型(符号付き整数型) <br>$\set{ x | x \in \mathbb{Z} , -(2^{n-1}) \leqq x \leqq 2^{n-1}-1}$
  - unsigned int 型(符号なし整数型)<br>$\set{ x | x \in \mathbb{N} , 0 \leqq x \leqq 2^{n-1}-1}$
- float 型(浮動小数点型)<br>型サイズの bit 数を$n=\set{32, 64}$<br>
  $\set{ x | x \in \mathbb{R} \lor \set{NaN} , (f_{min} \leqq x \leqq f_{max}) \lor \set{NaN}}$
- bool 型(論理値型)<br>$\set{ x | x=\set{0, 1}}$

## トレイト境界:

ジェネリック(T など)に特定の振る舞いをもつあらゆる型になり得ることを指定すること。<br>

### コード例:

```rust
// エラーが出るlargest関数
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        // エラーを回避するためにトレイト境界が必要
        // intとcharの場合、トレイト境界はOrd + Copyで十分
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 正常に動くlargest関数
// トレイト境界でジェネリックTがPartialOrd(狭義半順序関係)+所有権がCopyであることを明記
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

### トレイト境界の種類:自信ないよ(笑)。間違っていたら情報をください:

1. Eq(同値関係)[^1]<br>
   集合 $A$ 上の二項関係 $R$ について、以下の命題が全部真であるとき、関係 $R$ は同値関係である。

   - $\forall x \in A. xRx$ (反射律)
   - $\forall x, y \in A. (xRy \to yRx)$ (対称律)
   - $\forall x, y, z \in A. (xRy \land yRz \to xRz)$ (推移律)

2. PartialEq(部分同値関係)[^2]<br>
   集合 $A$ 上の二項関係 $R$ について、以下の命題が全部真であるとき、関係 $R$ は部分同値関係である。

   - $\forall x, y \in A. (xRy \to yRx)$ (対称律)
   - $\forall x, y, z \in A. (xRy \land yRz \to xRz)$ (推移律)

3. Ord(狭義全順序関係)[^3]<br>
   集合 $A$ 上の二項関係 $R$ について、以下の命題が全部真であるとき、関係 $R$ は全順序関係である。

   - $\forall x, y \in A. (xRy \lor yRx \lor x=y)$ (三分律)
   - $\forall x, y, z \in A. (xRy \land yRz \to xRz)$ (推移律)

4. PartialOrd(狭義半順序関係)[^4]<br>
   集合 $A$ 上の二項関係 $R$ について、以下の命題が全部真であるとき、関係 $R$ は半順序関係である。
   - $\forall x, y \in A. (xRy \to \lnot yRx)$ (非対称律)
   - $\forall x, y, z \in A. (xRy \land yRz \to xRz)$ (推移律)

[^1]: https://doc.rust-lang.org/std/cmp/trait.Eq.html
[^2]: https://en.wikipedia.org/wiki/Partial_equivalence_relation
[^3]: https://ja.wikipedia.org/wiki/%E5%85%A8%E9%A0%86%E5%BA%8F
[^4]: https://en.wikipedia.org/wiki/Partial_equivalence_relation
