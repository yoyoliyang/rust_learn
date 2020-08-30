fn main() {
    // vector不同于数组array，数组是固定的，vector可以修改长度
    // Vectors 只能存放相同类型的数据（可以利用枚举存放不同类型数据）
    let mut my_vector: Vec<u8> = Vec::new();
    my_vector.push(1);
    my_vector.push(3);
    my_vector[1] = 10;

    let two: &u8 = &mut my_vector[1]; // vector被引用后，不可修改
                                      // my_vector[1] = 7;  紧接着运行这段的话会错误！
    println!("{}", two);
    my_vector[1] = 7;
    println!("{}", my_vector[1]);
    // &mut two += 1;

    let my_vector_1 = vec![1, 2, 3, 4];

    println!("{}, {}, {}", my_vector[0], my_vector_1[0], my_vector.len());

    let v = vec![1, 2, 3, 4, 5];

    let r1: &i32 = &v[0]; // 引用v[0];

    match v.get(0) {
        // v.get返回的是一个Option
        Some(r1) => println!("{}", r1),
        None => println!("nothing found!"),
    }
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    println!("{}", first); // first引用后最后一次使用，可以修改vector集合
    v[0] = 10;
    println!("{}", v[0]);

    // 遍历vector,方法1
    for (index, i) in v.iter().enumerate() {
        println!("{}\t{}", index, i);
    }
    for i in &v {
        println!("{}", i);
    }

    enum Contact {
        phone(u32),
        name(String),
        sex(bool),
    }

    let people = vec![Contact::name(String::from("liyang")), Contact::sex(true)];
}
