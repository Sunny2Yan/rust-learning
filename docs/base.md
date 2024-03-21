

```rust
// 基础知识

// 1. 变量与可变性
// 变量(variables)：Rust 鼓励利用不可变性。当变量不可变时，一旦值被绑定一个名称上，你就不能改变这个值。可以在变量名前添加 mut 来使其可变。
// 常量(constants)：绑定到一个名称的不允许改变的值，且不允许对常量使用 mut。声明常量使用 const 关键字而不是 let，并且必须注明值的类型。Rust 对常量的命名约定是在单词之间使用全大写加下划线。
// 隐藏(Shadowing)：定义一个与之前变量同名的新变量，则称第一个变量被第二个隐藏。

fn var(){
    let mut x = 5;  // 变量
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;  // 常量

    let x = x + 1;  // 这个变量x隐藏了上面的x，此时不需要mut（mut是可改变变量值，而隐藏是创建新的变量）

    // 隐藏与mut的区别：
    let spaces_1 = "   ";
    let spaces_1 = spaces_1.len();  // 4

    let mut spaces_2 = "   ";
    // spaces_2 = spaces_2.len();  // 报错，不能改变变量类型
}

// 注：对于声明后未使用的变量，Rust 会给出警告，因为这可能会是个 BUG，可以使用下划线开头避免警告，如 `_x`。


// 2. 数据类型
// Rust 的数据类型(data type)分为两类：标量(scalar)和复合(compound)。Rust 是静态类型（statically typed）语言，即在编译时就必须知道所有变量的类型。

// 2.1 标量类型
// 整型：有符号：`i8`、`i16`、`i32`、`i64`、`i128`（数字为 bit），无符号：`u8`、`u16`、`u32`、`u64`、`u128`（数字为 bit），默认是 `i32`;
// 浮点型：`f32`、`f64`，默认是 `f64`（所有浮点型都是有符号的）；
// 布尔型：`true`，`false`，
// 字符类型：用单引号声明 `char` 字面量，双引号声明字符串字面量，`char` 类型的大小为 4-bit（Unicode）

fn scalar_type(){
    let sum = 5 + 10;  // addition
    let difference = 95.5 - 4.3;  // subtraction
    let product = 4 * 30;  // multiplication
    let quotient = 56.7 / 32.2;  // division

    let truncated = -5 / 3;  // 结果为 -1
    let remainder = 43 % 5;  // remainder

    let x = true;
    let y: bool = false; // with explicit type annotation
}

// 2.2 复合类型
// 元组(tuple)：元组长度固定，元组中的每个元素的类型不必相同，用 `tup` 声明元组类型；
// 数组(array)：数组长度固定，数组中的每个元素的类型必须相同，vector 类型是标准库提供的允许增长和缩小长度的类似数组的集合类型，一般不确定长度的用 vector；

fn compound(){
    // 2.2.1 元组
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;  // 解构元组，注意(.)
    let x = tup.0;  // 索引访问
    // 注：不带任何值的元组有个特殊的名称，叫做 单元（unit） 元组，其对应类型写作()，表示空

    // 2.2.2 数组（长度固定，存在栈上）
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];
    let a: [i32; 5] = [1, 2, 3, 4, 5];  // ; 5表示元素数量
    let a = [3; 5];  // 初始值加分号再加元素个数 [3, 3, 3, 3, 3]

    let first = a[0];  // 索引访问
}


// 3. 函数
// Rust 用 `fn` 关键字来声明新函数，函数名使用小写字母并用下划线分隔单词。Rust 返回值时不需要对返回值命名，但要在箭头（`->`）后声明它的类型。

// 1. 带参数
fn another_function(key: char, value: i32) {  // 参数类型必须指明
    println!("{key}: {value}");
}  // Rust 不关心函数定义所在的位置，只要函数被调用时出现在调用之处可见的作用域内就行

fn plus_one(x: i32) -> i32 {
    x + 1  // 没有分号，完整的语句不会返回值
}

fn add(x: i32) -> i32 {
    if x > 10 {
        return x * 2
    }
    x + 5
}

fn main() {
    println!("Hello, world!");
    var();
    scalar_type();
    compound();

    // 1. 带参数
    another_function('x', 5);

    // 2. 赋值
    let y = {  // 不能 y=x=3
        let x = 3;  // 语句需要 ;结尾
        x + 1     // 表达式不需要 ; 结尾
    };
    println!("The value of y is: {y}");

    // 3. 返回值
    let x = plus_one(5);
    println!("The value of x is: {x}");

    // 4. 提前返回要用 return
    let y = add(8);
    println!("The value of y is: {y}");
}


// 4. 控制流
// 4.1 `if` 表达式
fn if_control(){
    if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 3, or 2");
    }

// let语句可以使用if表达式，但数据类型必须相同
    let number = if condition { 5 } else { 6 };
}

// 4.2 循环
fn for_cycle(){
    // 1. loop 关键字表示无限循环直到明确要求停止
    loop {
        println!("again!");  // 一直输出again!
    }

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;  // break后再将counter值乘2
        }
    };

    // 2. while循环
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }

    // 3. for循环
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }
}