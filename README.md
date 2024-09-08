# rustup
- rustup update 更新rust版本

# cargo 

- cargo fmt  格式代码
- cargo fix  代码修复


# 并发
```
对于高级语言，只支持全部解决方案的一部分是完全可以理解的设计策略。因为高级语言往往会通过放弃部分控制能力
来获得有益于用户的抽象。但是，底层语言则被期望在任意场景下都可以提供一套性能最佳的解决方案，
并对硬件建立尽可能少的抽象。因此Rust提供了多种建模问题的工具来应对不同的场景和需求。
```

- 如何创建线程来同时运行多段代码
- 使用通道在线程间发送消息的消息传递式并发
- 允许多个线程访问同一片数据的共享状态式并发
- Sync trait 与 Send trait, 能够将Rust的并发保证从标准库中提供的类型扩展至用户自定义类型。


# 编译
```
rustc rust_print.rs
```

# 参考
https://rustwiki.org/zh-CN/rust-by-example/hello.html

# rust框架

- Rocket
- Actix-web
- Hyper
- Tokio runtime

## Tokio runtime

 cargo install mini-redis 

 https://tokio.rs/tokio/tutorial/hello-tokio


## Rocket

https://rocket.rs/v0.5-rc/guide/getting-started/


## Actix-web

https://actix.rs/

# rust文章

- https://mp.weixin.qq.com/s/fNeasOwibFz0_Ahjb4FiGQ
- https://mp.weixin.qq.com/s/AXXJnFdwYDiy5vfZ-fvVDQ

# git
```
git rm -r --cached **/target
git commit -m "Update .gitignore to ignore target directories"
```
