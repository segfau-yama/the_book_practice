# Rust The book 写経メモ

なんとなく分からなかったところをメモしていきます。

## トレイト境界

1. Eq(同値関係)<br>
   集合 $A$ 上の二項関係 $R$ について、以下の命題が全部真であるとき、関係 $R$ は同値関係である。

   - $\forall x \in A. xRx$ (反射律)
   - $\forall x \in A. \forall y \in A. (xRy \to yRx)$ (対称律)
   - $\forall x \in A. \forall y \in A. \forall z \in A. (xRy \land yRz \to xRz)$ (推移律)

2. PartialEq(部分同値関係?)<br>
   集合 $A$ 上の二項関係 $R$ について、以下の命題が全部真であるとき、関係 $R$ は部分同値関係である。

   - $\forall x \in A. \forall y \in A. (xRy \to yRx)$ (対称律)
   - $\forall x \in A. \forall y \in A. \forall z \in A. (xRy \land yRz \to xRz)$ (推移律)

3. Ord(全順序関係)<br>
   集合 $A$ 上の二項関係 $R$ について、以下の命題が全部真であるとき、関係 $R$ は全順序関係である。

   - $\forall x \in A.\forall y \in A. (xRy \lor yRx)$ (完全律)
   - $\forall x \in A. \forall y \in A. (xRy \land yRx \to x=y)$ (反対称律)
   - $\forall x \in A. \forall y \in A. \forall z \in A. (xRy \land yRz \to xRz)$ (推移律)

4. PartialOrd(半順序関係)<br>
   集合 $A$ 上の二項関係 $R$ について、以下の命題が全部真であるとき、関係 $R$ は半順序関係である。
   - $\forall x \in A. xRx$ (反射律)
   - $\forall x \in A. \forall y \in A. (xRy \land yRx \to x=y)$ (反対称律)
   - $\forall x \in A. \forall y \in A. \forall z \in A. (xRy \land yRz \to xRz)$ (推移律)
