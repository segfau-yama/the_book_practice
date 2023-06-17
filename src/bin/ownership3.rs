fn main() {
    // intの所有権はCopy
    let x = 5;
    let y = x;

    // 値がコピーされたためエラーは出ない
    println!("x = {}, y = {}", x, y);
}
