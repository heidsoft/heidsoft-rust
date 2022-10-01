/// 参数格式化输出
pub fn run(){
    println!("Hello from the print.rs file");

    println!("Number: {}",1);

    println!("{} is  from {}","Jake","Ether");

    println!("{0} 获取对应顺序的参数 {1}  {0} {2}","Jake","Ether","Rust");

    println!("{name} 使用名称获取参数 {action}",name="heidsoft", action="Play");
}