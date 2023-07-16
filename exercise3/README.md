# 学生管理系统

这是一个用Rust编写的简单学生管理系统。该系统包括对学生、班级、课程和社团的基本管理操作，包括创建、读取、更新和删除（CRUD）这些实体。此外，这个系统还提供了数据持久化的功能，可以将当前的数据状态保存到文件，也可以从文件中恢复数据状态。

## 功能

- 学生、班级、课程和社团的创建、读取、更新和删除操作。
- 数据持久化：将当前的数据状态保存到文件，以便在程序重新启动时恢复数据状态。

## 使用说明

首先，需要创建一个`StudentManagementSystem`对象，然后可以使用该对象的方法进行各种操作。例如：

```rust
let mut sms = StudentManagementSystem::new();

sms.add_student(Student {
    id: 1,
    name: "Alice".to_string(),
    class_id: 101,
});

sms.add_club(Club {
    id: 1,
    name: "Computer Club".to_string(),
    member_ids: vec![1],
});

sms.save_to_file("data.json").unwrap();
```

在这个例子中，我们创建了一个新的学生管理系统，添加了一个学生和一个社团，然后将数据保存到了`data.json`文件。

我们也可以从一个文件中加载数据：

```rust
let sms = StudentManagementSystem::load_from_file("data.json").unwrap();
```

## 依赖

- Rust 1.40.0 或更高版本
- [serde ↗](https://crates.io/crates/serde) 1.0.0 或更高版本
- [serde_json ↗](https://crates.io/crates/serde_json) 1.0.0 或更高版本

## 开源许可

该项目采用 MIT 许可证。