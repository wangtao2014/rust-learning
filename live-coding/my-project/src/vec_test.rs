pub fn vec_new() {
    let mut vec: Vec<i32> = Vec::new();

    // method 2
    // let vec2 = vec![1, 2, 3, 4, 5];

    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);

    println!("vec_new: {:?}", vec);
}

pub fn vec_read() {
    // 索引 panic
    // get 返回 None
    let vec = vec![1, 2, 3, 4, 5];
    let third = &vec[2];
    println!("the third element is: {:?}", third);

    match vec.get(2) {
        Some(third) => println!("the third element is: {:?}", third),
        None => println!("there is no third element"),
    }
}

pub fn vec_borrow() {
    let vec = vec![1, 2, 3, 4, 5];
    let first = &vec[0];

    // vec.push(6);
    println!("the first element is: {:?}", first);
}

pub fn vec_iter() {
    let mut vec = vec![1, 2, 3, 4, 5];
    for i in &mut vec {
        *i += 50;
    }

    for i in &vec {
        println!("{}", i);
    }
}
