mod minesweeper;

use minesweeper::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C"{
    fn alert(s: &str);
}

#[wasm_bindgen(js_name = GetMinesweeper)]
pub fn get_minesweeper() -> JsValue{
    let minesweeper = Minefield::new(16, 16, 80).expect("Error");

    JsValue::from_serde(&minesweeper.fields).unwrap()
}