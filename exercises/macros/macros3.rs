// macros3.rs
//
// Make me compile, without taking the macro out of the module!
//
// Execute `rustlings hint macros3` or use the `hint` watch subcommand for a
// hint.

#[macro_use] // 方法2. 在 crate 根作用域中引入 macros 模块中的所有宏
mod macros {
    //#[macro_export]  // 方法1. 使用 macro_export 属性将宏提升到 crate 的根作用域
    macro_rules! my_macro {
        () => {
            println!("Check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}
