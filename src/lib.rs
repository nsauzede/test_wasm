extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;


#[no_mangle]
pub extern fn add(x: u32, y: u32) -> u32 {
    x + y
}

#[no_mangle]
pub extern fn sub(x: u32, y: u32) -> u32 {
    x - y
}

// Reverse a string coming from JS 
#[wasm_bindgen]
pub fn reverse(s: String) -> String {
    s.chars().rev().collect::<String>()
}
