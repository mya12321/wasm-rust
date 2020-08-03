use wasm_bindgen::prelude::*;
use md2::{Md2, Digest};
use md4::Md4;
use md5::Md5;
use hex;

#[wasm_bindgen]
pub fn calcmd2(input: &str) {
    let mut s = String::from(input);
    for i in 0..100000000 {
      s = hex::encode(Md2::digest(&s.into_bytes()));
    }
    println!("{}", &s);
}

#[wasm_bindgen]
pub fn calcmd4(input: &str) {
    let mut s = String::from(input);
    for i in 0..100000000 {
      s = hex::encode(Md4::digest(&s.into_bytes()));
    }
    println!("{}", &s);
}

#[wasm_bindgen]
pub fn calcmd5(input: &str) {
    let mut s = String::from(input);
    for i in 0..100000000 {
      s = hex::encode(Md5::digest(&s.into_bytes()));
    }
    println!("{}", &s);
}
