# 函数
Rust 用 `fn` 关键字来声明新函数，函数名使用小写字母并用下划线分隔单词。Rust 返回值时不需要对返回值命名，但要在箭头（`->`）后声明它的类型。

## 1. 一般函数
```rust
// 1. 带参数
fn another_function(key: char, value: i32) {  // 参数类型必须指明
    println!("{key}: {value}");
}  // Rust 不关心函数定义所在的位置，只要函数被调用时出现在调用之处可见的作用域内就行

fn plus_one(x: i32) -> i32 {
    x + 1  // 没有分号，完整的语句不会返回值
}

fn add(x: i32) -> i32 {
    if x > 10 {
        return x * 2  // 对于提前返回一般使用 return
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
```

## 2. 匿名函数
```rust
let num = 5;
let plus_num = |x: i32| x + num;  // 使用闭包创建匿名函数

let mut num = 5;
{
    let mut add_num = move |x: i32| num += x;   // 闭包通过move获取了num的所有权
    add_num(5);
}
```

## 3. 发散函数
发散函数（diverging function）是rust中的一个特性。即，发散函数不返回，使用感叹号!作为返回类型表示。

```rust
fn diverging() -> ! {
  panic!("This function will never return");
}
```

## 4. 高阶函数

### 4.1 函数作为参数
```rust
fn inc(n: i32) -> i32 {
  n + 1
}

fn dec(n: i32) -> i32 {
  n - 1
}

fn process(n: i32, func: fn(i32) -> i32) -> i32 {
  func(n)
}
```

### 4.2 函数作为返回值
```rust
fn get_func(n: i32) -> fn(i32) -> i32 {
    fn inc(n: i32) -> i32 {
        n + 1
    }

    fn dec(n: i32) -> i32 {
        n - 1
    }

    if n % 2 == 0 {
        inc
    } else {
        dec
    }
}
```