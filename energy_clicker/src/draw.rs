#![warn(clippy::all)]
// #![feature(alloc)]


use stm32f7_discovery::{
    lcd::FramebufferAl88,
    lcd::FramebufferArgb8888,
    // lcd::Lcd,
    lcd::Color,
    lcd::Layer,
    // lcd::TextWriter,
};

// use stm32f7::stm32f7x6::{Peripherals};
use core::fmt::Write;
use crate::bmp_reader;

// static blue: Color = Color{red: 0,green: 0 ,blue: 255,alpha: 255};
// static green: Color = Color{red: 0,green: 255 ,blue: 0,alpha: 255};
static BLACK: Color = Color{red: 0,green: 0 ,blue: 0,alpha: 255};
static WHITE: Color = Color{red: 255,green: 255 ,blue: 255,alpha: 255};
// static grey: Color = Color{red: 127,green: 127 ,blue: 127,alpha: 127};
static YELLOW: Color = Color{red: 255, green: 255, blue: 0, alpha: 255};
static RED: Color = Color{red: 255, green: 0, blue: 0, alpha: 255};
static SKY_BLUE: Color = Color{red: 51, green: 204, blue: 255, alpha:255};


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

pub fn draw_circle(layer: &mut Layer<FramebufferArgb8888>, centre_x: usize, centre_y: usize, radius: usize, color: Color) {
    for x in centre_x-radius..centre_x+radius {
        for y in centre_y-radius..centre_y+radius {
            if dist(x, y, centre_x, centre_y) == radius {
                layer.print_point_color_at(x, y, color);
            }
        }
    }
}

pub fn draw_rectangle(layer: &mut Layer<FramebufferArgb8888>, x_coord: usize, y_coord: usize, x_length: usize, y_length: usize, color: Color) {
    let x_limit = x_coord + x_length + 1;
    let y_limit = y_coord + y_length + 1;
    for x in x_coord..x_limit {
            for y in y_coord..y_limit {
                if (x == x_coord || x == x_coord + x_length) && (y >= y_coord && y <= y_coord + y_length) {
                    layer.print_point_color_at(x, y, color);
                                    
                }
                if (y == y_coord || y == y_coord + y_length) && (x >= x_coord && x <= x_coord + x_length) {
                    layer.print_point_color_at(x, y, color);
                    
                }
            }
        }
}


pub fn color_rectangle(layer: &mut Layer<FramebufferArgb8888>, x_coord: usize, y_coord: usize, x_length: usize, y_length: usize, color: Color) {
    let x_limit = x_coord + x_length + 1;
    let y_limit = y_coord + y_length + 1;
    for x in x_coord..x_limit {
            for y in y_coord..y_limit {
                if (x > x_coord && x < x_coord + x_length) && (y > y_coord && y < y_coord + y_length) {
                    layer.print_point_color_at(x, y, color);
                                    
                }
                // if (y == y_coord || y == y_coord + y_length) && (x >= x_coord && x <= x_coord + x_length) {
                //     layer.print_point_color_at(x, y, color);
                    
                // }
            }
        }
}



// pub fn color_circle(mut layer: &mut Layer<FramebufferArgb8888>, centre_x: usize, centre_y: usize, radius: usize, color: Color) {
//     for x in centre_x-radius..centre_x+radius {
//         for y in centre_y-radius..centre_y+radius {
//             if dist(x, y, centre_x, centre_y) < radius {
//                 layer.print_point_color_at(x, y, color);
//             }
//         }
//     }
// }

pub fn write_string(layer: &mut Layer<FramebufferAl88>, x_coord: u32, y_coord: u32, text: core::fmt::Arguments<>){
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

pub fn draw_emissions(mut layer_1: &mut Layer<FramebufferArgb8888>, emissions: usize, max_emissions : usize) {
    color_rectangle(&mut layer_1, 10, 10, emissions, 20, RED);
    color_rectangle(&mut layer_1, 10 + emissions, 10, max_emissions - emissions, 20, WHITE);
    bmp_reader::draw_co2_text(layer_1, 220, 11, emissions);
}


pub fn draw_mode0(mut layer_1: &mut Layer<FramebufferArgb8888>, layer_2: &mut Layer<FramebufferAl88>, emissions: usize) {
    let max_x = 480;
    let max_y = 272;
    let centre_x = max_x / 2;
    let centre_y = max_y / 2;
    let radius = 50;

    layer_1.clear();
    layer_2.clear();   
    bmp_reader::draw_image(&mut layer_1, "background", 0, 0);            
    // draw_circle(&mut layer_1, centre_x, centre_y, radius, YELLOW);
    bmp_reader::draw_image(layer_1, "plants", 50, 80);
    //draw_rectangle(&mut layer_1, 50, 80, 100, 100, black);
    //draw_rectangle(&mut layer_1, 330, 80, 100, 100, BLACK);
    bmp_reader::draw_image(layer_1, "infrastructure", 330, 80);
    draw_rectangle(&mut layer_1, 10, 10, emissions, 20, RED);
    draw_rectangle(&mut layer_1, 9, 9, emissions + 2, 22, RED);
    
    // color_rectangle(&mut layer_1, 10, 10, emissions, 20, RED);
    
}

pub fn draw_mode1(layer_1: &mut Layer<FramebufferArgb8888>, layer_2: &mut Layer<FramebufferAl88>) {
    layer_1.clear();
    layer_2.clear();                

    bmp_reader::draw_image(layer_1, "back", 0, 0);

    bmp_reader::draw_image(layer_1, "solar", 120, 5);
    bmp_reader::draw_image(layer_1, "wind", 240, 5);
    bmp_reader::draw_image(layer_1, "gas", 360, 5);

    bmp_reader::draw_image(layer_1, "coal", 120, 135);
    bmp_reader::draw_image(layer_1, "nuclear", 240, 135);
    bmp_reader::draw_image(layer_1, "water", 360, 135);


    //draw_rectangle(&mut layer_1, 50, 20, 100, 100, black);
    //draw_rectangle(&mut layer_1, 170, 20, 100, 100, black);
    //draw_rectangle(&mut layer_1, 290, 20, 100, 100, black);
    
    //draw_rectangle(&mut layer_1, 20, 170, 440, 50, black);

}


pub fn draw_mode2(layer_1: &mut Layer<FramebufferArgb8888>, layer_2: &mut Layer<FramebufferAl88>, hidden: bool) {
    layer_1.clear();
    layer_2.clear();                

    bmp_reader::draw_image(layer_1, "back", 0, 0);

    bmp_reader::draw_image(layer_1, "battery", 120, 5);
    bmp_reader::draw_image(layer_1, "smartgrid", 240, 5);
    bmp_reader::draw_image(layer_1, "ac", 360, 5);

    bmp_reader::draw_image(layer_1, "dc", 120, 135);
    bmp_reader::draw_image(layer_1, "tree", 240, 135);
    if hidden {
        bmp_reader::draw_image(layer_1, "questionmark", 360, 135);
    }
    else {
        bmp_reader::draw_image(layer_1, "europapark", 360, 135);
    }
    


    //draw_rectangle(&mut layer_1, 50, 20, 100, 100, black);
    //draw_rectangle(&mut layer_1, 170, 20, 100, 100, black);
    //draw_rectangle(&mut layer_1, 290, 20, 100, 100, black);
    
    //draw_rectangle(&mut layer_1, 20, 170, 440, 50, black);

}

