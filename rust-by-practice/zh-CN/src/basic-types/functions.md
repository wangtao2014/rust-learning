# å½æ°
1. ððð
```rust,editable

fn main() {
    // ä¸è¦ä¿®æ¹ä¸é¢ä¸¤è¡ä»£ç !
    let (x, y) = (1, 2);
    let s = sum(x, y);

    assert_eq!(s, 3);
}

fn sum(x, y: i32) {
    x + y;
}
```


2. ðð
```rust,editable
fn main() {
   print();
}

// ä½¿ç¨å¦ä¸ä¸ªç±»åæ¥æ¿ä»£ i32
fn print() -> i32 {
   println!("hello,world");
}
```


3. ððð

```rust,editable
// ç¨ä¸¤ç§æ¹æ³æ±è§£
fn main() {
    never_return();
}

fn never_return() -> ! {
    // å®ç°è¿ä¸ªå½æ°ï¼ä¸è¦ä¿®æ¹å½æ°ç­¾å!
    
}
```

4. ðð åæ£å½æ°( Diverging function )ä¸ä¼è¿åä»»ä½å¼ï¼å æ­¤å®ä»¬å¯ä»¥ç¨äºæ¿ä»£éè¦è¿åä»»ä½å¼çå°æ¹
```rust,editable

fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // TODO
        }
        _ => {
            // TODO
        }
    };
    
    // è¿éä¸å¶è¿åä¸ä¸ª Noneï¼ä¸å¦ä½¿ç¨åæ£å½æ°æ¿ä»£
    never_return_fn()
}

// ä½¿ç¨ä¸ç§æ¹æ³å®ç°ä»¥ä¸åæ£å½æ°
fn never_return_fn() -> ! {
    
}
```

5. ðð
```rust,editable

fn main() {
    // å¡«ç©º
    let b = __;

    let _v = match b {
        true => 1,
        // åæ£å½æ°ä¹å¯ä»¥ç¨äº `match` è¡¨è¾¾å¼ï¼ç¨äºæ¿ä»£ä»»ä½ç±»åçå¼
        false => {
            println!("Success!");
            panic!("we have no value for `false`, but we can panic")
        }
    };

    println!("Exercise Failed if printing out this line!");
}
```

> ä½ å¯ä»¥å¨[è¿é](https://github.com/sunface/rust-by-practice/blob/master/solutions/basic-types/functions.md)æ¾å°ç­æ¡(å¨ solutions è·¯å¾ä¸) 
