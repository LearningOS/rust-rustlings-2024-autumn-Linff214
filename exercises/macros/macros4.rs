#[rustfmt::skip]
macro_rules! my_macro {
    () => {
        println!("Check out my macro!");
    };  // 在这里加上分号
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(7777);
}
