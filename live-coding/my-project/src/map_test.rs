use std::collections::HashMap;

// HashMap 用的比较少，不在 prelude 中
// HashMap 是同构的，k必须是同一类型，v必须是同一类型

pub fn map_new() {
    let mut map = HashMap::new();
    map.insert(String::from("blue"), 20);
    println!("{:#?}", map);
}

pub fn map_collect() {
    let teams = vec![String::from("blue"), String::from("green")];
    let initial_scores = vec![20, 40];

    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:#?}", scores);
}

pub fn map_borrow() {
    let field_name = String::from("Favorite color");
    let field_value = String::from("Red");

    let mut teams = HashMap::new();
    teams.insert(&field_name, &field_value);

    // borrow of moved value: `field_value` value borrowed here after move
    // println!("{}, {}", field_name, field_value);

    println!("map: {:#?}, {}, {}", teams, field_name, field_value);
}

pub fn map_obtain() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 30);
    scores.insert(String::from("green"), 40);

    let filed_name = String::from("blue");
    let value = scores.get(&filed_name);

    match value {
        Some(s) => println!("{}", s),
        None => println!("team not exist"),
    }
}

pub fn map_iter() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 30);
    scores.insert(String::from("green"), 40);

    for (k, v) in &scores {
        println!("{}, {}", k, v);
    }
}

pub fn map_update() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 30);
    scores.insert(String::from("blue"), 40);

    println!("{:?}", scores);
}

pub fn map_entry() {
    let mut scores = HashMap::new();

    scores.insert(String::from("blue"), 30);

    scores.entry(String::from("blue")).or_insert(40);
    scores.entry(String::from("green")).or_insert(50);

    println!("{:?}", scores);
}

pub fn map_entry_update() {
    let text = "hello world wonderful world";
    let mut scores = HashMap::new();

    for word in text.split_whitespace() {
        let count = scores.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:#?}", scores);

}