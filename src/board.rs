use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
pub enum Difficulty {
    Easy,
    Hard,
}

#[wasm_bindgen]
#[repr(u8)]
pub enum BoardSize {
    Standard,
    Large,
}