use std::collections::LinkedList;   //std library: linkedlist allows vector Command modifiction 
use piston_window::{Context, G2d};   //graph buffer
use piston_window::types::Color;     //color 

use crate::shapes1:: {make_shape};

const SNAKE_COLOR: Color = [0.0, 0.0, 1.0, 1.0];  //making the color of the snake [red, green, blue, opasity]
#[derive(Copy, Clone, PartialEq)]
pub enum Command {    //Defining directions 
    Up,
    Down,
    Left,
    Right,
}

impl Command {     //impl == Defning method for the correspindin commands, and direction , 
    pub fn opposite(&self) -> Command {     //Counter reverse not allowed class
        match *self {
            Command::Up => Command::Down,    //while going up cant go down... by passing back down 
            Command::Down => Command::Up,    //wiseversa
            Command::Left => Command::Right,  //going left, cant go right...by passing back right 
            Command::Right => Command::Left,   //wiseversa
        }
    }
}
#[derive(Debug, Clone)]    //struct for block type 
struct Block {
    x: i32,
    y: i32,
}

pub struct Snake {
    command: Command,  //takes in command from Command 
    S_body: LinkedList<Block>,    //Define the S_body and set of blocks 
    tail: Option<Block>,     //mutable, ever changing 
}

impl Snake {     //implementing the snake 
    pub fn new(x: i32, y: i32) -> Snake {    //Method of coord for snake 
        let mut S_body: LinkedList<Block> = LinkedList::new();    //mutable S_body (changing )
        S_body.push_back(Block {  //Push_back , appends an element to the back of the list 
            x: x + 2,    //Default snake with length of 3
            y,
        });
        S_body.push_back(Block {
            x: x + 1,
            y,      //second being x+1    
        });
        S_body.push_back(Block {
            x,    //first default ebing x&y 
            y,
        });     //resulting in a horizontal snake 

        Snake {
            command: Command::Right,    //by default moving to the right 
            S_body,
            tail: None,     //default lenght (3blocks)
        }
    }

    pub fn make(&self, con: &Context, g: &mut G2d) {
        for block in &self.S_body {
            make_shape(SNAKE_COLOR, block.x, block.y, con, g);     //each block witch contains the snake 
        }
    }

    pub fn head_position(&self) -> (i32, i32) {    //Defning head (where move leading is)
        let head_block = self.S_body.front().unwrap();  //head_block, referring to the first element , unwrap gets rid of enum without error 
        (head_block.x, head_block.y)    
    }

    pub fn move_forward(&mut self, dir: Option<Command>) {    //multable snake moving forward with commands 
        match dir {       
            Some(d) => self.command = d,
            None => (),
        }

        let (last_x, last_y): (i32, i32) = self.head_position();  // defning the posttion of head 

        let new_block = match self.command {   // Defnifng movement in   block terms  
            Command::Up => Block {
                x: last_x,
                y: last_y - 1,        // given the down is + y axis while up is - 
            },
            Command::Down => Block {
                x: last_x,
                y: last_y + 1,
            },
            Command::Left => Block {
                x: last_x - 1,
                y: last_y,
            },
            Command::Right => Block {
                x: last_x + 1,
                y: last_y,
            },
        };
        self.S_body.push_front(new_block);  //snake move, rf the last block and add a new block to the front. 
        let removed_block = self.S_body.pop_back().unwrap(); //pop back the linkedlist
        self.tail = Some(removed_block);     //removed from the end 
    }

    pub fn head_Command(&self) -> Command {   // method for getting the head position 
        self.command
    }

    pub fn next_head(&self, dir: Option<Command>) -> (i32, i32) {  //getting headposition 
        let (head_x, head_y): (i32, i32) = self.head_position();

        let mut moving_dir = self.command;    //mutable moving firection from the command 
        match dir {  //applying the command given
            Some(d) => moving_dir = d,
            None => {}
        }

        match moving_dir {   //Rescenter and accuracy 
            Command::Up => (head_x, head_y - 1),
            Command::Down => (head_x, head_y + 1),
            Command::Left => (head_x - 1, head_y),
            Command::Right => (head_x + 1, head_y),
        }
    }
//grow snake
    pub fn restore_tail(&mut self) {    // getting big similar method as tail but will do if apple is intercepted 
        let blk = self.tail.clone().unwrap();
        self.S_body.push_back(blk);
    }
//lose condition 
    pub fn overlap_tail(&self, x: i32, y: i32) -> bool {   
        let mut ch = 0;
        for block in &self.S_body {
            if x == block.x && y == block.y {  //if overlaps with s_body
                return true;
            }

            ch += 1;
            if ch == self.S_body.len() - 1 {   // checking for so that the head and tail interception dont cause losing condition, so 
                // len-1 ignoring 1 block where the head and tail momentarily overlap 
                break;      
            }
        }
        return false;
    }
}