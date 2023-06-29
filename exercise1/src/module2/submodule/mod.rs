// 直接打印会输出几个非字母字符
// pub fn print_a_to_z() {
//     for c in 'A' as u8..='z' as u8 {
//         println!("{}", c as char);
//     }
// }

// 下面是只打印字母字符的版本
pub fn print_a_to_z() {
    for c in 'A' as u8..='Z' as u8 {
        print!("{} ", c as char);
    }
    for c in 'a' as u8..='z' as u8 {
        print!("{} ", c as char);
    }
}
