use piston_window::types::Color;
use piston_window::*;

use rand::{thread_rng, Rng};  //rang  

use crate::shapes1::{make_shape, make_rectangle};  //shapes(blocks)
use crate::The_snake::{Command, Snake};   //commands

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];  //color food 
const BORDER_COLOR: Color = [1.0, 1.0, 1.0, 1.0];      //border color
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];  //gm screen color

const MOVING_PERIOD: f64 = 0.2;    //how fast snake move fps 
const RESTART_TIME: f64 = 1.0;      //game paused for 1 sec and over 

pub struct Rule {   //VBuilfing the game 
    snake: Snake,    //snake in 

    food_exists: bool,    //food in 
    food_x: i32,          //coor for food 
    food_y: i32,

    width: i32,     //area 
    height: i32,

    game_over: bool,    //gameover in 
    waiting_time: f64,     //w8 preiod for restart in 
}
  
impl Rule {    //implimneting all the "ins "
    pub fn new(width: i32, height: i32) -> Rule {    //new game 
        Rule {    //with all the "rules"
            snake: Snake::new(2, 2),    //where snake starts 
            waiting_time: 0.0,    //move immidiatley 
            food_exists: true,    
            food_x: 6,
            food_y: 4,   //food is there and in this coord 
            width,
            height,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {      //what key does what, and if key pressed 
        if self.game_over {    //if gameover out else 
            return;
        }

        let dir = match key {    
            Key::Up => Some(Command::Up),
            Key::Down => Some(Command::Down),     //arrows for direction 
            Key::Left => Some(Command::Left),
            Key::Right => Some(Command::Right),
            _ => Some(self.snake.head_Command()),
        };

        if let Some(dir) = dir {
            if dir == self.snake.head_Command().opposite() {  //opp direction wont work (quit out)
                return;
            }
        }

        self.update_snake(dir);
    }

    pub fn make(&self, con: &Context, g: &mut G2d) {
        self.snake.make(con, g);

        if self.food_exists {    //make food 
            make_shape(FOOD_COLOR, self.food_x, self.food_y, con, g);
        }

        make_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);    //border outline with graphics 
        make_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        make_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        make_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        if self.game_over {
            make_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {    //wotj rating time , 
        self.waiting_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {      //if food eaten == not exist => new food 
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);           //snake dir
        }
    }

    fn check_eating(&mut self) {         //snake with food overlap 
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Command>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1..self.width - 1);    //rng for food generation 
        let mut new_y = rng.gen_range(1..self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {    // food does not spawn on snake 
            new_x = rng.gen_range(1..self.width - 1);
            new_y = rng.gen_range(1..self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_snake(&mut self, dir: Option<Command>) {    //multable direction if alive 
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {                //reset of game method 
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.food_exists = true;
        self.food_x = 6;
        self.food_y = 4;
        self.game_over = false;
    }
}