fn main() {
    // 数据类型，整型，浮点，元组，数组
    // 元组和数组都可修改内容，但是长度不能修改
    let x = 100; // 默认i32整型
    let f = 1.234; // 默认f64双精度浮点
    let x: i64 = 45;
    let f: f64 = 6.7;
    let b: bool = false;

    // 表达式，但是x+1后面没有分号，所以是一个返回值
    let y = {
        let x = 10;
        x + 1 // 无分号，返回了x+1的数据
    };
    // 返回值（函数）, 无分好
    fn five() -> i32 {
        5
    }

    println!("{}", y);

    let c = 'h'; // char类型, char在c语言中是单字节，在rust中是4个字节,utf-8编码
    let c: char = '中';

    // 元组,长度不可变
    let mut tup: (i32, f64, u8) = (500, 6.4, 1); // 元组类型，可定义多个数据类型
    tup.0 = 10; // 修改元组内数据
    let (x, y, z) = tup; // x=500,y=6.4,z=1
    println!("{}", tup.0); // 索引访问

    // 数组,长度不可变
    let arr = [1, 2, 3, 4, 5]; // array只能定义一种数据类型, 数组长度是固定的！不同于vector
    let arr: [i32; 5] = [1, 2, 3, 4, 5]; // 注意[]中的分号
    let mut arr: [String; 2] = ["Yes".to_string(), "No".to_string()];
    arr[0] = "YYes".to_string();
    println!("{:?}", arr);

    let name: String;
    name = "oceanlee".to_string();

    let n = 100_000_000; // 可以使用分隔符来方便阅读

    println!("{}, {}, {}, {}", x, f, b, name);
}
