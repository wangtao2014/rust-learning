# æ³å

### å½æ°
1. ððð
```rust,editable

// å¡«ç©º
struct A;          // å·ä½çç±»å `A`.
struct S(A);       // å·ä½çç±»å `S`.
struct SGen<T>(T); // æ³å `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
    // ä½¿ç¨éæ³åå½æ°
    reg_fn(__);          // å·ä½çç±»å
    gen_spec_t(__);   // éå¼å°æå®ç±»ååæ°  `A`.
    gen_spec_i32(__); // éå¼å°æå®ç±»ååæ°`i32`.

    // æ¾å¼å°æå®ç±»ååæ° `char`
    generic::<char>(__);

    // éå¼å°æå®ç±»ååæ° `char`.
    generic(__);
}
```

2. ðð 
```rust,editable

// å®ç°ä¸é¢çæ³åå½æ° sum
fn sum

fn main() {
    assert_eq!(5, sum(2i8, 3i8));
    assert_eq!(50, sum(20, 30));
    assert_eq!(2.46, sum(1.23, 1.23));
}
```


### ç»æä½å `impl`

3. ð
```rust,editable

// å®ç°ä¸ä¸ªç»æä½ Point è®©ä»£ç å·¥ä½


fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
}
```

4. ðð
```rust,editable

// ä¿®æ¹ä»¥ä¸ç»æä½è®©ä»£ç å·¥ä½
struct Point<T> {
    x: T,
    y: T,
}

fn main() {
    // ä¸è¦ä¿®æ¹è¿è¡ä»£ç ï¼
    let p = Point{x: 5, y : "hello".to_string()};
}
```

5. ðð
```rust,editable

// ä¸º Val å¢å æ³ååæ°ï¼ä¸è¦ä¿®æ¹ `main` ä¸­çä»£ç 
struct Val {
    val: f64,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}


fn main() {
    let x = Val{ val: 3.0 };
    let y = Val{ val: "hello".to_string()};
    println!("{}, {}", x.value(), y.value());
}
```

### æ¹æ³
6. ððð 

```rust,editable
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    // å®ç° mixupï¼ä¸è¦ä¿®æ¹å¶å®ä»£ç ï¼
    fn mixup
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = Point { x: "Hello", y: 'ä¸­'};

    let p3 = p1.mixup(p2);

    assert_eq!(p3.x, 5);
    assert_eq!(p3.y, 'ä¸­');
}
```

7. ðð
```rust,editable

// ä¿®å¤éè¯¯ï¼è®©ä»£ç å·¥ä½
struct Point<T> {
    x: T,
    y: T,
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let p = Point{x: 5, y: 10};
    println!("{}",p.distance_from_origin())
}
```

> ä½ å¯ä»¥å¨[è¿é](https://github.com/sunface/rust-by-practice/blob/master/solutions/generics-traits/generics.md)æ¾å°ç­æ¡(å¨ solutions è·¯å¾ä¸) 

