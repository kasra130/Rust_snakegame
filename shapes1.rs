use piston_window::{rectangle, Context, G2d};  //rec simple rect fun, g2d graphics , 
use piston_window::types::Color;            //color allowed 
const BLOCK_SIZE: f64 = 25.0;      //constant size , with f64 annot by 25 pxl

pub fn to_coord(coord: i32) -> f64 {   //function for game coordinates casted as f64
    (coord as f64) * BLOCK_SIZE
}
pub fn to_coord_u32(coord: i32) -> u32 {   //last minute add since the coord function needs to be called agin in u32 because presition is way off without 
    to_coord(coord) as u32
}
pub fn make_shape(color: Color, x: i32, y: i32, con: &Context, g: &mut G2d)   {   //making (color block, passing x&y valuyes , context passed, and a mutable G2D GUI graphics)

    let win_x = to_coord(x);      //passing the single block coord to the "to_coord" function (converted now to f64)
    let win_y = to_coord(y);

    rectangle(          //Define graphical block as a rectangle which has :      
        color,                 //color
        [win_x, win_y, BLOCK_SIZE, BLOCK_SIZE],  //accpts coord, size
        con.transform,      //graphics buffer 
        g,
    );
}



pub fn make_rectangle(    //same parameters as make_shape
    color: Color,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    con: &Context,
    g: &mut G2d,
) {
    let x = to_coord(x);
    let y = to_coord(y);

    rectangle(     //rectangle size is controlled, and will used for the size of the window 
        color,
        [
            x,
            y,
            BLOCK_SIZE * (width as f64),
            BLOCK_SIZE * (height as f64),
        ],
        con.transform,
        g,
    );
}

