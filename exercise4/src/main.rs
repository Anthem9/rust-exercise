// 使用枚举包裹三个不同的类型
enum Message {
    String(String),
    I32(i32),
    F64(f64),
}

// 使用trait object
trait Print {
    fn print(&self);
}

struct StringWrapper(String);
impl Print for StringWrapper {
    fn print(&self) {
        println!("{}", self.0);
    }
}

struct I32Wrapper(i32);
impl Print for I32Wrapper {
    fn print(&self) {
        println!("{}", self.0);
    }
}

struct F64Wrapper(f64);
impl Print for F64Wrapper {
    fn print(&self) {
        println!("{}", self.0);
    }
}

fn main() {
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
    let objects = vec![
        Box::new(StringWrapper("hello".to_string())) as Box<dyn Print>,
        Box::new(I32Wrapper(42)) as Box<dyn Print>,
        Box::new(F64Wrapper(3.14)) as Box<dyn Print>,
    ];

    for obj in objects {
        obj.print();
    }
}

// 两种方法的区别:
// 1. 枚举方法可以直接存储不同类型的值,trait object需要先用struct封装后转为trait对象
// 2. 枚举匹配每个分支可以直接使用原始类型,trait object在调用方法时需要通过trait定义的接口
// 3. 枚举存储具体类型可以进行优化,trait object需要动态分发带来额外的运行时开销
// 4. 枚举可以明确表达类型,trait object是把具体类型抹去只保留行为
// 5. 通常来说,相比trait object,枚举能达到更好的性能;但trait object提供更好的扩展性和抽象能力