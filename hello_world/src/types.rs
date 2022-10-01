/// 类型定义
pub fn run(){
    // 默认 is "i32"
    let x = 1;

    // 默认 is "f64"
    let y = 2.5;

    // 增加 explicit type 
    let z: i64 = 454545454545;

    //find max size
    println!("Max i32: {}",std::i32::MAX); 
    println!("Max i64: {}",std::i64::MAX); 

    // Boolean
    let is_active: bool = true;

    // get boolean from expression
    let is_greater: bool = 10 < 5;

    let a1 = 'a';

    // 笑脸
    let face ='\u{1F600}';

    println!("{:?}",(x,y,z,is_active,is_greater,a1,face));
}