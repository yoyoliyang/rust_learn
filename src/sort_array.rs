fn main() {
    let mut arr1 = vec![34, 12, 45, 7, 8, 4, 31, 19, 15, 21, 13];
    let mut arr2: [i32] = Vector::new();
    let mut count = 0;
    // while count <= arr1.len() {
    let r_arr1 = &mut arr1;
    for (index, _item) in r_arr1.iter().enumerate() {
        // println!("{}\t{}", index, item);
        count += 1;
        if r_arr1[index + 1] > r_arr1[index] {
            let a = arr1[index];
            arr1[index] = arr1[index + 1];
            arr1[index + 1] = a;
        }
    }
    println!("{:?}", arr1);
}
