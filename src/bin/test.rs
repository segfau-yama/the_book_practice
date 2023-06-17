// この場合、引数firstとsecondは同じライフタイムを持っています。
// Rustはできるだけ短いライフタイムを推測しようとします。
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>`はライフタイム`'a`は「少なくとも」`'b`と同じくらいの長さを持つと読みます。
// この場合`&'a i32`というライフタイムの参照を受け取りますが、Coercionの結果`&'b i32`を返します。
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2; // ライフタイムはsecondより長い

    {
        let second = 3; // ライフタイムはfirstより短い

        println!("２つの数字の積は{}", multiply(&first, &second));
        println!(
            "{}が変数firstに入っています。",
            choose_first(&first, &second)
        );
    };
}
