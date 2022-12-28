use std::fmt::Display;

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("read more...{}", self.summarize_author())
    }

    fn obtain_context(&self) -> String {
        format!("hello content")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("author: {}", self.obtain_context())
    }

    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("username: {}", self.username)
    }

    fn summarize(&self) -> String {
        format!("{}, {}", self.username, self.content)
    }
}

pub fn notify(item: impl Summary) {
    println!("Breaking new summary: {}", item.summarize());
}

pub fn notify2(item1: impl Summary, item2: impl Summary) {
    println!(
        "Breaking new summary: {}, {}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify1<T: Summary>(item: T) {
    println!("Breaking new summary: {}", item.summarize());
}

pub fn notify3<T: Summary>(item1: T, item2: T) {
    println!(
        "Breaking new summary: {}, {}",
        item1.summarize(),
        item2.summarize()
    );
}

pub fn notify4<T: Summary + Display, U: Clone + Display>(item1: T, item2: U) {
    println!("Breaking new summary: {}", item1.summarize());
}

pub fn notify5<T, U>(item1: T, item2: U) -> String
where
    T: Summary + Display,
    U: Clone + Display,
{
    println!("Breaking new summary: {}", item1.summarize());
    "hello world".to_string()
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self { 
        Self { x, y } 
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("the largest member is x = {}", self.x);
        } else {
            println!("the largest member is y = {}", self.y);
        }
    }
} 
