use std::fmt::{Error, Display, Formatter, Write};
use rand::prelude::*;
use std::fmt;
use std::borrow::{BorrowMut};
use crate::minesweeper::State::{Hidden, Flagged, Revealed};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Minefield{
    pub fields : Vec<Vec<Field>>
}

#[derive(Debug,Serialize, Deserialize)]
pub struct Field{
    is_bomb: bool,
    state: State,
    neighbour_bombs: i8
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum State{
    Hidden,
    Flagged,
    Revealed
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Position {
    x : usize,
    y : usize
}


impl Minefield{
    pub fn new(
        width: usize,
        height : usize,
        bombs : usize
    ) -> Result<Minefield, Error> {
        let mut rng = rand::thread_rng();
        let mut bomb :Vec<Position> = Vec::with_capacity(bombs);

        for _ in 0..bombs{
            let w = rng.gen_range(0..width);
            let h = rng.gen_range(0..height);

            bomb.push(Position{x : w, y : h})
        }


        Ok(Minefield{
            fields: Minefield::generate_2d_field_vec(width, height, bomb).expect("Incorrect input")
        })
    }

    pub fn click_field(&mut self, position: Position) -> Result<(), Error>{
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
            return Err(Error::default())
        }

        if !is_clickable{
            return Ok(())
        }

        let bombs = self.check_neighbouring_bombs(&position);

        self.fields[position.y][position.x].reveal(bombs);

        Ok(())
    }

    fn check_neighbouring_bombs(&self , position :&Position) -> i8{
        let mut bombs : i8 = 0;
        if position.x > 0 {

        }

        //Top-Left
        if position.y > 0 && position.x > 0 && self.fields[position.y - 1][position.x - 1].is_bomb {
            bombs += bombs;
        }
        //Mid-Left
        if position.x > 0  && self.fields[position.y][position.x - 1].is_bomb {
            bombs += bombs;
        }

        //Top-Mid
        if position.y > 0 && self.fields[position.y - 1][position.x].is_bomb{
            bombs += bombs;
        }

        if position.y > 0 {
            //Bot-Left
            if position.y - 1 < self.fields.len() && position.x > 0 && self.fields[position.y + 1][position.x - 1].is_bomb{
                bombs += bombs;
            }

            //Bot-Mid
            if position.y - 1 < self.fields.len() && self.fields[position.y + 1][position.x].is_bomb{
                bombs += bombs;
            }
        }

        if position.x > 0 {
            //Top-Right
            if position.y > 0 && position.x - 1 < self.fields[0].len() && self.fields[position.y - 1][position.x + 1].is_bomb{
                bombs += bombs;
            }

            //Mid-Right
            if position.x - 1 < self.fields[0].len() && self.fields[position.y][position.x + 1].is_bomb{
                bombs += bombs;
            }
        }

        if position.x > 0 && position.y > 0 {
            //Bot-Right
            if position.y - 1 < self.fields.len() && position.x - 1 < self.fields[0].len() && self.fields[position.y + 1][position.x + 1].is_bomb{
                bombs += bombs;
            }
        }

        bombs
    }

    pub fn lost_game(& mut self, position :Position){

    }

    pub fn flag_field(& mut self, position: Position) -> Result<(), Error>{
        let field = self.fields[position.y][position.x].borrow_mut();

        match field.state {
            State::Revealed => Err(Error::default()),
            State::Flagged => Ok(field.state = Hidden),
            State::Hidden => Ok(field.state = Flagged)
        }.expect("Cannot flag Revealed field");

        Ok(())
    }

    fn generate_2d_field_vec(
        width: usize,
        height : usize,
        bombs : Vec<Position>
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
        bombs: &Vec<Position>
    ) -> Result<Vec<Field>, Error> {

        if size <= 0 {
            return Err(Error::default());
        }

        let mut fields : Vec<Field> = Vec::with_capacity(size);

        for pos_x in 0..size {
            let is_bomb = bombs.iter().find(|x| **x == Position{x: pos_x, y : pos_y}).is_some();

            fields.push(Field::new(is_bomb))
        }

        Ok(fields)
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