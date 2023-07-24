# 枚举与Trait Object的比较

此项目展示了如何在Rust中使用枚举和Trait Object来处理多种类型的值。此项目包含两个示例：一个使用枚举，另一个使用Trait Object。

## 示例

### 使用枚举

在这个示例中，我们定义了一个`Message`的枚举，它可以是`String`、`i32`或`f64`类型的值。我们在向量中存储了这三种类型的值，并在循环中打印出这些值。

```rust
let messages = vec![
    Message::String("hello".to_string()),
    Message::I32(42),
    Message::F64(3.14),
];

for msg in &messages {
    match msg {
        Message::String(s) => println!("{}", s),
        Message::I32(i) => println!("{}", i),
        Message::F64(f) => println!("{}", f),
    }
}
```

### 使用Trait Object

在这个示例中，我们定义了一个`Print` trait，该trait有一个`print`方法。然后我们为`StringWrapper`、`I32Wrapper`和`F64Wrapper`结构体实现了这个trait。我们在向量中存储了这三种类型的值，并在循环中调用它们的`print`方法。

```rust
let objects = vec![
    Box::new(StringWrapper("hello".to_string())) as Box<dyn Print>,
    Box::new(I32Wrapper(42)) as Box<dyn Print>,
    Box::new(F64Wrapper(3.14)) as Box<dyn Print>,
];

for obj in objects {
    obj.print();
}
```

## 区别

1. 枚举可以直接存储不同类型的值，而Trait Object则需要先用struct封装后转为Trait对象。
2. 在处理枚举值时，我们可以直接使用原始类型。但在处理Trait Object时，我们需要通过trait定义的接口。
3. 枚举可以进行优化，因为它存储了具体的类型。而Trait Object需要进行动态分发，这会带来额外的运行时开销。
4. 枚举可以明确表达类型，而Trait Object是把具体类型抹去，只保留行为。
5. 通常来说，相比Trait Object，枚举能达到更好的性能；但Trait Object提供更好的扩展性和抽象能力。

## 运行

要运行这个项目，你需要安装Rust。然后使用`cargo run`命令即可。