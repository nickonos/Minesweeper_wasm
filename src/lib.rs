mod minesweeper;

use minesweeper::*;
use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[wasm_bindgen]
extern "C"{
    fn alert(s: &str);
}

#[wasm_bindgen(js_name = GetMinesweeper)]
pub fn get_minesweeper() -> JsValue{
    let minesweeper = Minefield::new(30, 16, 80).expect("Error");

    JsValue::from_serde(&minesweeper.fields).unwrap()
}

#[wasm_bindgen(js_name = ClickField)]
pub fn click_field(value: &JsValue) -> JsValue {
    let input  = value.into_serde();
    //let mut minefield = Minefield::from_fields(input.fields.fields);
    //let result = minefield.click_field(Position{x : input.x, y: input.y});


    JsValue::from_serde(&js_value).unwrap()
}

#[wasm_bindgen(js_name = FlagField)]
pub fn flag_field(value: JsValue){

}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Input {
    fields : Minefield,
    y: usize,
    x: usize
}