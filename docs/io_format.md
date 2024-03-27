# 格式化输入、输出

## 1. 标准化输入

```rust
use std::io;

fn read_input() -> io::Result<()> {
    let mut input = String::new();

    io::stdin().read_line(&mut input)?;  // 此处的？等价于python中的try
    println!("You typed: {}", input.trim());
    Ok(())
}
```

## 2. 标准化输出
Rust 中常见的标准输出宏有 `print!` 和 `println!`。
- print!()  // 仅输出
- println!()  // 输出并换行

## 3. 文件输入

```rust
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    // 创建一个文件路径
    let path = Path::new("hello.txt");
    let display = path.display();  // 文件路径

    // 打开文件只读模式, 返回一个 `io::Result<File>` 类型
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // 文件输入数据到字符串，并返回 `io::Result<usize>` 类型
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, Error::description(&why)),
        Ok(_) => print!("{} contains:\n{}", display, s),
    }
}
```

## 4. 文件输出

```rust
use std::error::Error;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

fn main() {
    static LOREM_IPSUM: &'static str = "这是一段文本";

    let path = Path::new("out/lorem_ipsum.txt");
    let display = path.display();

    // 用只写模式打开一个文件，并返回 `io::Result<File>` 类型
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, Error::description(&why)),
        Ok(file) => file,
    };

    // 写入 `LOREM_IPSUM` 字符串到文件中, 并返回 `io::Result<()>` 类型
    match file.write_all(LOREM_IPSUM.as_bytes()) {
        Err(why) => {
            panic!("couldn't write to {}: {}", display, Error::description(&why))
        },
        Ok(_) => println!("successfully wrote to {}", display),
    }
}
```

## 5. format格式

```rust
// key-value的值只能出现在position值之后(同python的函数参数)
let s = format!("{1}是个有着{0:>0width$}KG重，{height:?}cm高的大胖子", 
    81, "wayslog", width=4, height=178);
print!("{}", s);  // wayslog是个有着0081KG重，178cm高的大胖子
```