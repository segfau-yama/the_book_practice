fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

// 'aはライフタイム注釈という
// 'aを入れることで関数内で引数の寿命が短い方(x or y)に寿命を合わせる。
// ライフタイムが実行事に変わる場合、明記する
// どちらかが死んだ場合道連れになる。無理心中構文
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
