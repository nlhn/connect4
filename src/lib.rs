use wasm_bindgen::prelude::*;
// mod connect4;
// mod board;
// use crate::connect4::{ Connect4Board, Connect4AI };
// use crate::board::*;



#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn play_connect4(size: i32, mode: i32) {
    alert("Hello, connect4!");
}