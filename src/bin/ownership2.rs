fn main() {
    let s1 = String::from("hello");
    // s1からs2にStringをコピー
    let s2 = s1.clone();

    // Stringがコピーされているためエラーは出ない
    println!("s1 = {}, s2 = {}", s1, s2);
}
