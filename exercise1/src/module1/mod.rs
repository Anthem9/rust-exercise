// pub fn print_a_to_z() {
//     for c in ('a' as u8..='Z' as u8).filter(|&c| c.is_ascii()) {
//         println!("{}", c as char);
//     }
// }
// 因为字符a的ASCII码值比Z大，所以上述函数不会打印任何东西。故修改为下面的函数

pub fn print_a_to_z() {
    for c in 'a' as u8..='z' as u8 {
        println!("{}", c as char);
    }
    for c in 'A' as u8..='Z' as u8 {
        println!("{}", c as char);
    }

}
