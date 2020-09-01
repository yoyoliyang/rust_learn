fn main() {
    let mut list1 = vec![1, 2, 5, 10, 4, 22, 19, 54, 13, 20];
    let mut list2 = Vec::new();

    let mut largest = list1[0];

    let list1_orgi_len = list1.len();
    let mut index = 0;
    let mut count = 0;
    while count < list1_orgi_len {
        for (_index, &i) in list1.iter().enumerate() {
            if i > largest {
                largest = i;
                index = _index;
            }
        }
        if index < list1.len() {
            list1.remove(index);
        }
        list2.push(largest);
        largest = list1[0];
        println!("count : {}", count);
        count += 1;
        println!("{:?}", list1);
        println!("{:?}", list2);
    }
}
