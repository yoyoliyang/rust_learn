fn main() {
    print_numbers_to(10);
}

fn print_numbers_to(num: i32) {
    for i in -1..num {
        // println!("{}", i);
        if is_even(i) {
            println!("{} 可以被整除", i);
        } else {
            println!("{} 不可以被整除", i);
        }
    }
}

// 箭头定义函数的数据类型
fn is_even(num: i32) -> bool {
    return num % 2 == 0;
}
