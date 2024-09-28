/**
 * Vec 是 Rust 标准库中的一个动态数组类型，提供了许多灵活和高效的操作。
 * Vec 的主要特征包括：
 * 1. 动态大小：Vec 可以存储任意大小的数据，而不像数组那样需要指定大小。
 * 2. 可变长度：Vec 的长度可以在运行时动态改变。
 * 3. 存储任意类型：Vec 可以存储任何类型的数据，包括基本类型、结构体、枚举等。
 */
fn main() {
    println!("Hello, world!");
    println!("创建 Vec");
    let mut numbers: Vec<i32> = Vec::<i32>::new();
    println!("创建空 Vec: {:?}", numbers);
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);
    println!("添加元素后 Vec: {:?}", numbers);

    demo_vec_init_1();
}
/**
 * 使用宏初始化 Vec
 * 1. vec![] 创建一个空的 Vec。
 * 2. vec![1, 2, 3] 创建一个包含三个元素的 Vec。
 */
fn demo_vec_init_1() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("初始化 Vec: {:?}", numbers);

    // 访问元素
    println!("第一个元素: {}", numbers[0]);

    // 修改元素
    numbers[1] = 25;
    println!("修改后的 Vec: {:?}", numbers);

    //遍历 Vec
    for number in &numbers {
        println!("元素: {}", number);
    }

    println!("循环后可继续使用: {:?}", numbers); // 这行代码会导致编译错误
}
