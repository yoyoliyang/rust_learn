// #[derive(Debug)]

fn main() {
    let vec1: Vec<u32> = vec![11, 23, 46, 67, 123, 6, 8, 9, 34, 10];
    let mut vec2 = Vec::new();
    let mut sum: u32 = 0;
    // let s = &mut sum;
    for item in &vec1 {
        println!("{}", item);
        sum += item;
    }
    println!("result: {}", sum as f64 / 2.0);

    let mut n = 0;
    loop {
        for (index, item) in vec1.iter().enumerate() {
            if vec1[index + 1] > vec[index] {
                (vec1[index], vec1[index + 1]) = (vec1[index + 1], vec1[index]);
            }
        }
    }
}
