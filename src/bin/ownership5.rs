fn main() {
    let s1 = gives_ownership();
    println!("{}", s1);

    let s2 = String::from("hello");

    // 関数に値がMove
    let s3 = takes_and_gives_back(s2);
    println!("{}", s3);
}
fn gives_ownership() -> String {
    let some_string = String::from("hello");

    // ここでsome_stringを戻り値代入先(s2)の変数にMove
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    // ここでsome_stringを戻り値代入先(s3)の変数にMove
    a_string
}
