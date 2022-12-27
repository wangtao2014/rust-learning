use std::collections::HashMap;

pub fn value_tour() {
    let authenticated = true;

    if authenticated {
        println!("authenticated = {}", authenticated);
    } else {
        todo!()
    }

    // modify value 
    let mut total = 0usize;
    total += 1;

    println!("total = {}", total);

    // pass to function
    let name = "Tao".to_string();
    print_my_name(name);

    // pass by ref
    let mut map: HashMap<String, String> = HashMap::new();
    #[allow(unused)]
    let my_map = &mut map;
    // print_map(&map);

    // map.insert("hello".into(), "world".into());
    // insert_map(my_map);

    // multithread
    // let mut data = vec![1, 2, 3];

    // thread::spawn(move || {
    //     data.push(5);
    // });

    // data.push(4);
}

#[allow(unused)]
fn print_my_name(name: String) {
    println!("name = {}", name);
}

#[allow(unused)]
fn print_map(map: &HashMap<String, String>) {
    todo!();
}

#[allow(unused)]
fn insert_map(map: &mut HashMap<String, String>) {
    todo!();
}