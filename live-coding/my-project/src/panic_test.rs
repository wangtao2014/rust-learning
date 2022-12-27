// 可恢复错误 Result<T, T>
// 不可恢复错误 panic! 宏
// RUST_BACKTRACE=1 cargo run
// RUST_BACKTRACE=full cargo run

pub fn panic_test() {
    panic!("crash test failed");
}