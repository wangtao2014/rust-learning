pub fn string_iter() {
    // string 不能按索引访问 s[0]
    let hello = String::from("中国人");
    let len = hello.len();
    println!("len-{}", len);

    // 字节
    for s in hello.bytes() {
        println!("S-{}", s);
    }

    // 标量值
    for t in hello.chars() {
        println!("T-{}", t);
    }

    let slice = &hello[0..3];
    println!("slice-{}", slice);
}