fn main() {
    // Stringの所有権はMove
    let s1 = String::from("hello");
    // s1からs2に所有権が移動
    let s2 = s1;

    // s1は既に参照先文字列を持っていないためエラー
    println!("{}, world!", s1);
}
