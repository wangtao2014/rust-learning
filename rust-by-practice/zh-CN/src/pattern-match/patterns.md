# æ¨¡å¼

1. ðð ä½¿ç¨ `|` å¯ä»¥å¹éå¤ä¸ªå¼, èä½¿ç¨ `..=` å¯ä»¥å¹éä¸ä¸ªé­åºé´çæ°å¼åºå
```rust,editable

fn main() {}
fn match_number(n: i32) {
    match n {
        // å¹éä¸ä¸ªåç¬çå¼
        1 => println!("One!"),
        // ä½¿ç¨ `|` å¡«ç©ºï¼ä¸è¦ä½¿ç¨ `..` æ `..=`
        __ => println!("match 2 -> 5"),
        // å¹éä¸ä¸ªé­åºé´çæ°å¼åºå
        6..=10 => {
            println!("match 6 -> 10")
        },
        _ => {
            println!("match 11 -> +infinite")
        }
    }
}
```

2. ððð  `@` æä½ç¬¦å¯ä»¥è®©æä»¬å°ä¸ä¸ªä¸æ¨¡å¼ç¸å¹éçå¼ç»å®å°æ°çåéä¸
```rust,editable

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    // å¡«ç©ºï¼è®© p å¹éç¬¬äºä¸ªåæ¯
    let p = Point { x: __, y: __ };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        // ç¬¬äºä¸ªåæ¯
        Point { x: 0..=5, y: y@ (10 | 20 | 30) } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }
}
```

3. ððð

```rust,editable

// ä¿®å¤éè¯¯
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello {
            id:  3..=7,
        } => println!("id å¼çèå´å¨ [3, 7] ä¹é´: {}", id),
        Message::Hello { id: newid@10 | 11 | 12 } => {
            println!("id å¼çèå´å¨ [10, 12] ä¹é´: {}", newid)
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
}
```

4. ðð å¹éå®å«ï¼match guardï¼æ¯ä¸ä¸ªä½äº match åæ¯æ¨¡å¼ä¹åçé¢å¤ if æ¡ä»¶ï¼å®è½ä¸ºåæ¯æ¨¡å¼æä¾æ´è¿ä¸æ­¥çå¹éæ¡ä»¶ã
```rust,editable

// å¡«ç©ºè®©ä»£ç å·¥ä½ï¼å¿é¡»ä½¿ç¨ `split`
fn main() {
    let num = Some(4);
    let split = 5;
    match num {
        Some(x) __ => assert!(x < split),
        Some(x) => assert!(x >= split),
        None => (),
    }
}
```

5. ððð ä½¿ç¨ `..` å¿½ç¥ä¸é¨åå¼
```rust,editable

// å¡«ç©ºï¼è®©ä»£ç å·¥ä½
fn main() {
    let numbers = (2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048);

    match numbers {
        __ => {
           assert_eq!(first, 2);
           assert_eq!(last, 2048);
        }
    }
}
```

6. ðð ä½¿ç¨æ¨¡å¼ `&mut V` å»å¹éä¸ä¸ªå¯åå¼ç¨æ¶ï¼ä½ éè¦æ ¼å¤å°å¿ï¼å ä¸ºå¹éåºæ¥ç `V` æ¯ä¸ä¸ªå¼ï¼èä¸æ¯å¯åå¼ç¨
```rust,editable

// ä¿®å¤éè¯¯ï¼å°½éå°å°ä¿®æ¹ä»£ç 
// ä¸è¦ç§»é¤ä»»ä½ä»£ç è¡
fn main() {
    let mut v = String::from("hello,");
    let r = &mut v;

    match r {
       &mut value => value.push_str(" world!") 
    }
}
```

> ä½ å¯ä»¥å¨[è¿é](https://github.com/sunface/rust-by-practice/blob/master/solutions/pattern-match/patterns.md)æ¾å°ç­æ¡(å¨ solutions è·¯å¾ä¸) 