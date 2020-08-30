fn main() {
    // rust定义：当一个堆数据被其他变量引用后，那么之前定义的变量失效。
    {
        let s = "hello";
        println!("{}", s);
    } // s在此时失效（作用域）

    let mut s = "hello"; // 字面值,定义到&str 栈中
                         // s.push_str("world!"); //错误 ，字面值不可变;
                         //
    let mut s = String::from("hello"); // 定义到堆中
    s.push_str("world!");
    println!("{}", s);

    let s1 = s; // s1 引用了s的内存指针,此时s失效
                // println!("{}", s); // 错误，s已经失效
    println!("{}", s1); // 正确

    // 使用clone函数，可以完全拷贝引用数据, 但是消耗资源
    let s2 = s1.clone();
    println!("{}, {}", s1, s2);

    // 可以Copy的数据类型(引用后但是之前赋值的变量不失效）：
    // i32, u32, f64, char和元组（仅限包含类型都是Copy）的
}
