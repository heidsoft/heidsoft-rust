/// 字符串测试
pub fn run(){
    let mut hello = String::from("Hello ");

    // Get length
    println!("Length: {} ", hello.len());

    // Push char
    hello.push('W');

    // Push string 
    hello.push_str("orld");

    // 容量获取
    println!("Capacity: {}", hello.capacity());

    // Check if empty
    println!("Is Empty: {}", hello.is_empty());

    // 包含
    println!("Contains 'World' {} ", hello.contains("World"));

    // 替换
    println!("Replace: {} ", hello.replace("World", "There"));
    println!("{}", hello);
}