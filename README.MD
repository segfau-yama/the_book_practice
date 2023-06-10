# Rust The book 写経メモ

なんとなく分からなかったところをメモしていきます。

## トレイト境界:

ジェネリック(T など)に特定の振る舞いをもつあらゆる型になり得ることを指定すること。<br>

### コード例:

```rust
// エラーが出るlargest関数
fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        // 二進型は比較演算子を使えないためエラー
        // エラーを回避するためにトレイト境界が必要
        if item > largest {
            largest = item;
        }
    }
    largest
}

// 正常に動くlargest関数
// トレイト境界でジェネリックTがPartialOrd(半順序関係)+所有権がCopyであることを明記
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

### トレイト境界の種類(間違いの可能性があるため訂正待ち):

1. Eq(同値関係)<br>
   集合 $A$ 上の二項関係 $R$ について、以下の命題が全部真であるとき、関係 $R$ は同値関係である。

   - $\forall x \in A. xRx$ (反射律)
   - $\forall x, y \in A. (xRy \to yRx)$ (対称律)
   - $\forall x, y, z \in A. (xRy \land yRz \to xRz)$ (推移律)

2. PartialEq(恐らく部分同値関係)<br>
   集合 $A$ 上の二項関係 $R$ について、以下の命題が全部真であるとき、関係 $R$ は部分同値関係である。

   - $\forall x, y \in A. (xRy \to yRx)$ (対称律)
   - $\forall x, y, z \in A. (xRy \land yRz \to xRz)$ (推移律)

3. Ord(全順序関係)<br>
   集合 $A$ 上の二項関係 $R$ について、以下の命題が全部真であるとき、関係 $R$ は全順序関係である。

   - $\forall x, y \in A. (xRy \lor yRx)$ (完全律)
   - $\forall x, y \in A. (xRy \to yRx)$ (対称律)
   - $\forall x, y \in A. (xRy \land yRx \to x=y)$ (反対称律)
   - $\forall x, y, z \in A. (xRy \land yRz \to xRz)$ (推移律)

4. PartialOrd(恐らく狭義半順序関係)<br>
   集合 $A$ 上の二項関係 $R$ について、以下の命題が全部真であるとき、関係 $R$ は半順序関係である。
   - $\forall x, y \in A. (xRy \to \lnot yRx)$ (非対称律)
   - $\forall x, y, z \in A. (xRy \land yRz \to xRz)$ (推移律)
