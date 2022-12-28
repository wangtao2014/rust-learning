fn largest(list: &[i32]) -> i32 {
    let mut largest_num = list[0];
    for &item in list {
        if item > largest_num {
            largest_num = item;
        }
    }
    largest_num
}

struct Point<T> {
    x: T,
    y: T,
}

// 把 T 放在 impl 关键字后，表示在类型 T 上实现方法
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// 只针对具体类型实现方法（其余类型没有实现该方法）
impl Point<i32> {
    fn x1(&self) -> &i32 {
        &self.x
    }
}

struct Pointx<T, U> {
    x: T,
    y: U,
}

// struct 里的泛型类型参数可以和方法的泛型类型参数不同
impl<T, U> Pointx<T, U> {
    fn mixup<V, W>(self, other: Pointx<V, W>) -> Pointx<T, W> {
        Pointx {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let number_list = vec![10, 20 , 30, 40, 50];
    let largest_number = largest(&number_list);
    println!("largest_number: {:?}", largest_number);

    let point = Point { x: 4.4, y: 5.5 };
    // no method named `x1` found for struct `Point<{float}>` in the current scope
    // println!("{}", point.x1());

    println!("{}", point.x());

    let point1 = Pointx {x: 2, y: 3};
    let point2 = Pointx {x:"hello", y:'c'};
    let point3 = point1.mixup(point2);

    println!("point3.x = {}, point3.y = {}", point3.x, point3.y);
}