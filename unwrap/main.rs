use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

/**
 * (base) ➜  unwrap git:(master) ✗ cargo r
   Compiling unwrap v0.1.0 (/Users/heidsoft/Downloads/research/rust/heidsoft-rust/unwrap)
    Finished dev [unoptimized + debuginfo] target(s) in 2.70s
     Running `target/debug/unwrap`
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "No such file or directory" }', main.rs:7:52
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
 */
fn main() {
    // 尝试打开一个可能不存在的文件
    let file = File::open("non_existent_file.txt").unwrap();

    let reader = BufReader::new(file);
    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}
