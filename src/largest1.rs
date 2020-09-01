fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let sorted = hashMap::new();

    // println!("the largest number is :{}", largest(&number_list));
}

fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    // 注意此处的表示方法，list是一个引用，故 使用&item的方式，但是在函数体内，需要使用item
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}
