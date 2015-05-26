extern crate opencc;

use opencc::OpenCC;

#[cfg(not(test))]
fn main() {
    let cc = OpenCC::new("t2s.json");
    println!("{}", cc.convert("乾坤一擲"));
}