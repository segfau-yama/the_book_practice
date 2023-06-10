// トレイト境界はT型に何らかの型が代入された際の制約
// PartialOrd: 半順序集合(反射律、推移律、反対称律を満たす)
// Copy: 所有権がコピーである
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![3, 5, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![0.33, 0.55, 2.5, 0.6547, 3.21];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
