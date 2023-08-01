macro_rules! say_hello {
    ($name:expr) => {
        println!("Hello, {}", $name);
    };
}

macro_rules! create_function {
    ($func_name:ident) => (
        fn $func_name() {
            println!("你调用了 {:?}()", stringify!($func_name));
        }
    )
}

create_function!(foo);
create_function!(bar);

macro_rules! test {
    ($left:expr; 和 $right:expr) => (
        println!("{:?} 和 {:?} 是 {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left && $right)
    );
    ($left:expr; 或 $right:expr) => (
        println!("{:?} 或 {:?} 是 {:?}",
                 stringify!($left),
                 stringify!($right),
                 $left || $right)
    );
}

fn main() {
    say_hello!("Rustacean");
    foo();
    bar();
    test!(1i32 > 0i32; 和 2i32 > 1i32);
    test!(true; 或 false);
}

