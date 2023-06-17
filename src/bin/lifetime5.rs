fn main() {
    // string1が生まれる
    let string1 = String::from("abcd");
    {
        // string2が生まれる
        let string2 = "xyz";

        let result = longest(string1.as_str(), string2);
        println!("The longest string is {}", result);
    }
    // string2が死亡
}
// string1が死亡。string2の方が寿命が長い

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
