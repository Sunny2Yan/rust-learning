# 变量
- 变量(variables)：当变量不可变时，一旦值被绑定一个名称上，就不能改变这个值。可以在变量名前添加 mut 来使其可变；
- 常量(constants)：声明常量使用 const 关键字，且必须注明值的类型。Rust 对常量的命名约定是在单词之间使用全大写加下划线；
- 隐藏(Shadowing)：定义一个与之前变量同名的新变量，则称第一个变量被第二个隐藏。

```rust
let x = 5; // 不可变变量
let mut y = 5;  // 可变变量
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;  // 常量
let x = x + 1;  // 这个变量x隐藏了上面的x，此时不需要mut（mut是可改变变量值，而隐藏是创建新的变量）

// 隐藏与mut的区别：
let spaces_1 = "   ";
let spaces_1 = spaces_1.len();  // 4

let mut spaces_2 = "   ";
// spaces_2 = spaces_2.len();  // 报错，不能改变变量类型

// 注：对于声明后未使用的变量，Rust 会给出警告，因为这可能会是个 BUG，可以使用下划线开头避免警告，如 `_x`。
```