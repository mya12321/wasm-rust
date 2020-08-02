use wasm_bindgen::prelude::*;
use md2::Md2;
use md4::Md4;
use md5::Md5;

#[wasm_bindgen]
pub fn md2(input: &str) -> &str {
    for i in 0..100000000 {
      input = Md2.digest();
    }
    println("{}", input);
}

#[wasm_bindgen]
pub fn md4(input: &str) -> &str {
    for i in 0..100000000 {
      input = Md2.digest();
    }
    println("{}", input);
}

#[wasm_bindgen]
pub fn md5(input: &str) -> &str {
    for i in 0..100000000 {
      input = Md2.digest();
    }
    println("{}", input);
}
