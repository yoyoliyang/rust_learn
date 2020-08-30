struct Color {
    red: u8,
    blue: u8,
    green: u8,
}

fn main() {
    // references
    let mut x = 10;
    let xr = &mut x;
    *xr += 1; // 解引用

    let arr1 = [1, 2, 3, 4];
    let r1 = &arr1[..2];
    println!("{:?}", r1);

    println!("{}", x);

    // blue进入作用域
    let blue = Color {
        red: 0,
        blue: 10,
        green: 200,
    };

    // 此处非blue进入作用域，而是指向blue的引用ref进入,故blue不会移出
    print_color(&blue); // &引用符号，指针

    let s1 = String::from("hello,world!"); // s1被引入
    let len = calculate_length(&s1);

    println!("The length of \"{}\" :{}", s1, len);

    // 可变引用
    let mut s2 = String::from("hello,world!"); // s2被引入
    let r1 = &mut s2; //  第一次引用，正确
                      // let r2 = &mut s2; // 第二次引用错误，因为rust存在数据竞争，只能引用一次
    println!("before: {}", r1);
    change_s2(r1); // 注意可变引用的方式
    println!("changed: {}", r1);

    let mut s3 = String::from("hello,world!"); // s3
    let r3 = &s3;
    let r4 = &s3;
    // let r5 = &mut s3; // 错误，引用不能可变和不可变混杂使用！
    println!("{} {}", r3, r4); // 最后一次使用引用，r3，r4不再使用
    let r5 = &mut s3; // 正确，因为r3和r4是最后一次引用使用
}

fn print_color(c: &Color) {
    println!("Color - R:{} ,G:{}, B:{}", c.red, c.green, c.blue);
}

fn calculate_length(s: &String) -> usize {
    s.len() // s离开，但是s无引用所有权，故s1不会丢失
}

fn change_s2(s: &mut String) {
    *s = "Hello liyang".to_string(); // *为解引用方式
    s.push_str(" oceanlee");
}

/*
fn dangle() {
    // 垂悬指针错误实例
    let s = String::from("hello");

    // &s 错误！当s离开作用域就会被清除
    s
}
*/
