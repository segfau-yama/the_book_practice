fn main() {
    let s = String::from("hello");
    // 関数に値がMove
    takes_ownership(s);
    // Moveしているためエラー
    // println!("{}", s);

    let x = 5;
    // 関数に値がCopy
    makes_copy(x);
    // Copyしているため問題なし
    println!("{}", x);
}
fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
