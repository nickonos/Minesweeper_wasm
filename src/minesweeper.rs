use std::fmt::{Error, Display, Formatter, Write};
use rand::prelude::*;
use std::fmt;
use std::borrow::{BorrowMut};
use crate::minesweeper::State::{Hidden, Flagged, Revealed};
use serde::{Serialize, Deserialize};
use wasm_bindgen::prelude::*;
use js_sys::*;
use std::iter::Iterator;
use web_sys::console;

#[wasm_bindgen]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Minefield{
    fields : Vec<Vec<Field>>,
    pub height : usize,
    pub width: usize,
    pub bombs: usize,
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Position {
    pub x : usize,
    pub y : usize
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Field{
    is_bomb: bool,
    state: State,
    neighbour_bombs: i8
}

#[wasm_bindgen]
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum State{
    Hidden,
    Flagged,
    Revealed
}

impl Minefield {
    fn check_neighbouring_bombs(&self, p:&Position) -> i8{
        let mut bombs : i8 = 0;

        //check lower bounds
        if p.x > 0 && self.fields[p.y][p.x - 1].is_bomb{
            bombs += 1;
        }

        if p.y > 0 && self.fields[p.y - 1][p.x].is_bomb{
            bombs += 1;
        }

        if p.x > 0 && p.y > 0 && self.fields[p.y - 1][p.x - 1].is_bomb{
            bombs += 1;
        }

        //check upper bounds
        if p.x < self.width - 1 && self.fields[p.y][p.x + 1].is_bomb{
            bombs += 1;
        }

        if p.y < self.height -1 && self.fields[p.y + 1][p.x].is_bomb{
            bombs += 1;
        }

        if p.x < self.width - 1 && p.y < self.height -1 && self.fields[p.y + 1][p.x + 1].is_bomb{
            bombs += 1;
        }

        //check corner bounds
        if p.x < self.width - 1 && p.y > 0 && self.fields[p.y -1][p.x + 1].is_bomb{
            bombs += 1;
        }

        if p.x > 0 && p.y < self.height - 1 && self.fields[p.y + 1][p.x - 1].is_bomb{
            bombs += 1;
        }

        bombs
    }

    pub fn lost_game(& mut self, position :Position){

    }

    fn generate_2d_field_vec(
        width: usize,
        height : usize,
        bombs : Vec<&Position>
    ) -> Result<Vec<Vec<Field>>, Error>{
        if height <= 0 {
            return Err(Error::default());
        }

        let mut fields :Vec<Vec<Field>> = Vec::with_capacity(height);

        for y in 0..height {
            fields.push(Minefield::generate_field_vec(width, y, &bombs).expect("Incorrect input"))
        }

        Ok(fields)
    }

    fn generate_field_vec(
        size: usize,
        pos_y: usize,
        bombs: &Vec<&Position>
    ) -> Result<Vec<Field>, Error> {

        if size <= 0 {
            return Err(Error::default());
        }

        let mut fields : Vec<Field> = Vec::with_capacity(size);

        for pos_x in 0..size {
            let is_bomb = bombs.iter().find(|x| ***x == Position{x: pos_x, y : pos_y}).is_some();

            fields.push(Field::new(is_bomb))
        }

        Ok(fields)
    }

    fn reveal_neighbouring_bombs(&mut self, p : &Position){
        //check lower bounds
        if p.x > 0{
            self.click_field(Position::new(p.x - 1, p.y));
        }

        if p.y > 0{
            self.click_field(Position::new(p.x, p.y - 1));
        }

        if p.x > 0 && p.y > 0 {
            self.click_field(Position::new(p.x - 1, p.y - 1));
        }

        //check upper bounds
        if p.x < self.width - 1{
            self.click_field(Position::new(p.x + 1, p.y));
        }

        if p.y < self.height -1{
            self.click_field(Position::new(p.x, p.y + 1));
        }

        if p.x < self.width - 1 && p.y < self.height - 1{
            self.click_field(Position::new(p.x + 1, p.y + 1));
        }

        //check corner bounds
        if p.x < self.width - 1 && p.y > 0 {
            self.click_field(Position::new(p.x + 1, p.y - 1));
        }

        if p.x > 0 && p.y < self.height - 1{
            self.click_field(Position::new(p.x - 1, p.y + 1));
        }

    }

}

#[wasm_bindgen]
impl Position{
    pub fn new (
        x : usize,
        y : usize
    ) -> Position{
        Position{
            x,
            y
        }
    }
}

#[wasm_bindgen]
impl Minefield{
    pub fn new(
        width: usize,
        height : usize,
        bombs : usize
    ) -> Minefield {
        let mut rng = rand::thread_rng();
        let mut bomb :Vec<&Position> = Vec::with_capacity(bombs);
        let mut fields : Vec<Position> = Vec::with_capacity(width * height);

        for y in 0..height{
            for x in 0..width{
                fields.push(Position::new(x ,y))
            }
        }

        for _ in 0..bombs{
            let i = rng.gen_range(0..fields.len());
            bomb.push(&fields[i])
        }


        Minefield{
            fields: Minefield::generate_2d_field_vec(width, height, bomb).expect("Incorrect input"),
            width,
            height,
            bombs
        }
    }

    pub fn get_minefield(&self) -> js_sys::Uint32Array{
        let mut uint_vec: Vec<u32> = Vec::new();

        for fieldarr in &self.fields {
            for field in fieldarr {
                match field.state {
                    Hidden => {
                        uint_vec.push(10)
                    }
                    Flagged => {
                        uint_vec.push(11);
                    }
                    Revealed => {
                        if field.is_bomb{
                            uint_vec.push(12)
                        }else{
                            uint_vec.push(field.neighbour_bombs as u32)
                        }

                    }
                }
            }
        }

        js_sys::Uint32Array::from(&uint_vec[..])
    }


    pub fn click_field(&mut self, position: Position) -> bool{
        let res = self.fields[position.y][position.x].is_clickable();

        let mut bomb : bool = false;

        let is_clickable = match res {
            Ok(v) => v,
            Err(_) => {
                bomb = true;
                false
            }
        };

        if bomb{
            self.lost_game(position);
            return false
        }

        if !is_clickable{
            return true
        }

        let bombs = self.check_neighbouring_bombs(&position);

        self.fields[position.y][position.x].reveal(bombs);

        if bombs == 0 {
            self.reveal_neighbouring_bombs(&position);
        }

        true
    }

    pub fn fields(&self) -> *const Vec<Field> {
        self.fields.as_ptr()
    }

    pub fn flag_field(& mut self, position: Position) -> bool{
        let field = self.fields[position.y][position.x].borrow_mut();

        match field.state {
            State::Revealed => Err(Error::default()),
            State::Flagged => Ok(field.state = Hidden),
            State::Hidden => Ok(field.state = Flagged)
        }.expect("Cannot flag Revealed field");

        true
    }
}

impl Display for Minefield {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for x in self.fields.iter(){
            for y in x.iter(){
                f.write_str(&*(y.display().to_owned() + " "));
            }
            f.write_char('\n')?;
        }

        Ok(())
    }
}

impl Display for State{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            State::Hidden => write!(f, "Hidden"),
            State::Flagged => write!(f, "Flagged"),
            State::Revealed => write!(f, "Revealed")
        }
    }
}

impl Field{
    pub fn new(is_bomb: bool) -> Field{
        Field{
            is_bomb,
            state: State::Hidden,
            neighbour_bombs: 0
        }
    }

    pub fn is_clickable(&self) -> Result<bool, Error>{
        if self.state == Flagged || self.state == Revealed{
            return Ok(false)
        }

        if self.is_bomb {
            return Err(Error::default())
        }

        Ok(true)
    }

    pub fn reveal(&mut self, neighbour_bombs: i8){
        self.neighbour_bombs = neighbour_bombs;
        self.state = Revealed;
    }

    pub fn display(&self) -> String {
        if self.state == Revealed && self.is_bomb{
            return "Bomb".to_string();
        }

        if self.state == Revealed && !self.is_bomb{
            return self.neighbour_bombs.to_string()
        }

        self.state.to_string()
    }

}


#[cfg(test)]
mod tests{
    use crate::minesweeper::{Minefield, Position};

    #[test]
    fn test(){
        let mut mf = Minefield::new(10, 12, 6).expect("Error setting up Minefield");

        mf.flag_field(Position{ x : 1, y: 1}).expect("Cannot Flag Field");

        mf.click_field(Position{x: 0, y: 0}).expect("Bomb");
        println!("{}", mf);

    }
}