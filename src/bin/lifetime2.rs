fn main() {
    {
        let r;

        {
            // xが生まれる
            let x = 5;
            r = &x;
            // 参照先xが死亡してないため正常終了
            println!("r: {}", r);
            // xが死亡
        }
    }
}
