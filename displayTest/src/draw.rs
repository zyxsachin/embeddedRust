#![warn(clippy::all)]
#![feature(alloc)]


use stm32f7_discovery::{
    lcd::FramebufferAl88,
    lcd::FramebufferArgb8888,
    lcd::Lcd,
    lcd::Color,
    lcd::Layer,
    lcd::TextWriter,
};

use stm32f7::stm32f7x6::{Peripherals};
use core::fmt::Write;

static blue: Color = Color{red: 0,green: 0 ,blue: 255,alpha: 255};
static green: Color = Color{red: 0,green: 255 ,blue: 0,alpha: 255};
static black: Color = Color{red: 0,green: 0 ,blue: 0,alpha: 255};
static white: Color = Color{red: 255,green: 255 ,blue: 255,alpha: 255};
static grey: Color = Color{red: 127,green: 127 ,blue: 127,alpha: 127};
static yellow: Color = Color{red: 255, green: 255, blue: 0, alpha: 255};
static red: Color = Color{red: 255, green: 0, blue: 0, alpha: 255};

// pub struct Drawer {
//     display:  lcd::Color,

// }

// impl Drawer {
   

//     pub fn new(display1: lcd::Color) -> Self {
//         Drawer {
//             display: display1,
//         }
        
//     }

//     pub fn init() {
//         let peripherals = Peripherals::take().unwrap();
//         let mut rcc = peripherals.RCC;
//         let mut ltdc = peripherals.LTDC;

//         let mut lcd = init::init_lcd(&mut ltdc, &mut rcc);
//     }

//     pub fn draw_circle() {
   
//     }


// }

pub fn draw_circle(mut layer: &mut Layer<FramebufferArgb8888>, centre_x: usize, centre_y: usize, radius: usize, color: Color) {
    for x in centre_x-radius..centre_x+radius {
        for y in centre_y-radius..centre_y+radius {
            if dist(x, y, centre_x, centre_y) == radius {
                layer.print_point_color_at(x, y, yellow);
            }
        }
    }
}

pub fn draw_rectangle(mut layer: &mut Layer<FramebufferArgb8888>, x_coord: usize, y_coord: usize, x_length: usize, y_length: usize, color: Color) {
    let x_limit = x_coord + x_length + 1;
    let y_limit = y_coord + y_length + 1;
    for x in x_coord..x_limit {
            for y in y_coord..y_limit {
                if (x == x_coord || x == x_coord + x_length) && (y >= y_coord && y <= y_coord + y_length) {
                    layer.print_point_color_at(x, y, black);
                                    
                }
                if (y == y_coord || y == y_coord + y_length) && (x >= x_coord && x <= x_coord + x_length) {
                    layer.print_point_color_at(x, y, black);
                    
                }
            }
        }
}



pub fn color_circle(mut layer: &mut Layer<FramebufferArgb8888>, centre_x: usize, centre_y: usize, radius: usize, color: Color) {
    for x in centre_x-radius..centre_x+radius {
        for y in centre_y-radius..centre_y+radius {
            if dist(x, y, centre_x, centre_y) < radius {
                layer.print_point_color_at(x, y, color);
            }
        }
    }
}

pub fn write_string(mut layer: &mut Layer<FramebufferAl88>, x_coord: u32, y_coord: u32, text: core::fmt::Arguments<>){
     let mut text_layer = layer.text_writer();
     text_layer.x_pos = x_coord as usize;
     text_layer.y_pos = y_coord as usize;
     text_layer.write_fmt(text);

}

fn dist (px : usize, py : usize, qx : usize, qy : usize) -> usize {
    let d_x;
    let d_y;
    if px > qx {
        d_x = px - qx;
    }
    else {
        d_x = qx - px;
    }
    if py > qy {
        d_y = py - qy; 
    }
    else {
        d_y = qy - py;
    }
    let t = d_x * d_x + d_y * d_y;
    //my_sqrt(t)
    for i in 0..t {
        if i * i >= t {
            return i;
        }
            
    } 
    0
}
