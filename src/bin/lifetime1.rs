fn main() {
    {
        let r;

        {
            // xが生まれる
            let x = 5;
            r = &x;
            // xが死亡
        }

        // 参照先がないためエラー
        println!("r: {}", r);
    }
}
