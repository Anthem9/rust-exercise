# Rust Complex Number Operations

这个项目展示了如何在Rust中为自定义类型实现加法运算，并使用Trait Object实现类型方法的调用。

## 实现加法运算

我们定义了一个复数（`Complex`）类型，并为它实现了加法运算。这是通过实现 `std::ops::Add` trait 完成的：

```rust
use std::ops::Add;

#[derive(Debug)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            real: self.real + other.real,
            imag: self.imag + other.imag,
        }
    }
}
```

## 使用Trait Object实现类型方法的调用

接下来，我们定义了一个 `Printable` trait，并为 `Complex` 类型实现了这个 trait：

```rust
trait Printable {
    fn print(&self);
}

impl Printable for Complex {
    fn print(&self) {
        if self.imag >= 0.0 {
            println!("Complex: {} + {}i", self.real, self.imag);
        } else {
            println!("Complex: {} - {}i", self.real, -self.imag);
        }
    }
}
```

我们创建了一个函数 `call_print`，它接收一个实现了 `Printable` trait 的 trait 对象作为参数，并调用它的 `print` 方法：

```rust
fn call_print(item: &dyn Printable) {
    item.print();
}
```

## 运行

要运行这个项目，你需要安装Rust。然后使用`cargo run`命令即可。

在主函数中，我们创建了两个 `Complex` 对象，执行了加法操作，并打印了结果：

```rust
fn main() {
    let c1 = Complex { real: 1.0, imag: 2.0 };
    let c2 = Complex { real: 2.0, imag: -3.0 };
    let result = c1 + c2;
    call_print(&result);
}
```

这个项目能帮助理解trait和trait object，以及如何为自定义类型实现运算符重载。