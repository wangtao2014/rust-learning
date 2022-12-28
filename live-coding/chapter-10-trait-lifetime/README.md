# 泛型
- 提高代码复用能力，处理重复代码的问题
    - 泛型是具体类型或其他属性的抽象代替
    - 模板，占位符
    - 编译时替换为具体的类型

- 函数中定义的泛型
    - fn largest<T>(list: &[T]) -> T {}

- 结构体中的泛型，字段持有泛型数据类型
    - struct Point<T> {x: T, y: T}
    - struct Point<T, U> {x: T, y: U}

- enum 中的泛型，变体持有泛型数据类型
    - enum Option<T> { Some(T), None }
    - enum Result<T, E> { Ok(T), Err(E) }

- 方法中的泛型
    - 为 struct 和 enum 中实现方法的时候，可在定义中使用泛型
    - 注意：
        - 把 T 放在 impl 关键字后，表示在类型 T 上实现方法
            - impl<T> Point<T>
        - 只针对具体类型实现方法（其余类型没有实现该方法）
            - impl Point<i32>
    - struct 里的泛型类型参数可以和方法的泛型类型参数不同

- 泛型代码的性能
    - 使用泛型和使用具体类型代码的性能一样
    - 单态化
        - 在编译时将泛型替换为具体的类型的过程

# Trait