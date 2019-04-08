
use stm32f7_discovery::{
    lcd::FramebufferArgb8888,
    lcd::Color,
    lcd::Layer,
};



pub struct Bmp {
    width : usize,
    height : usize,
    color : [Color; 32000],
}

pub const EXAMPLE: &[u8] = include_bytes!("../images/e2.bmp");
pub const BLITZ: &[u8] = include_bytes!("../images/Blitz.bmp");
pub const BLITZ2: &[u8] = include_bytes!("../images/Blitz2.bmp");
pub const PLANTS: &[u8] = include_bytes!("../images/powerpflanzen.bmp");
pub const COAL: &[u8] = include_bytes!("../images/coal.bmp");
pub const SOLAR: &[u8] = include_bytes!("../images/sun.bmp");
pub const WATER: &[u8] = include_bytes!("../images/damm.bmp");
pub const NUCLEAR: &[u8] = include_bytes!("../images/nuke.bmp");
pub const GAS: &[u8] = include_bytes!("../images/gas.bmp");
pub const WIND: &[u8] = include_bytes!("../images/wind.bmp");
pub const BACK: &[u8] = include_bytes!("../images/back.bmp");
pub const TEST: &[u8] = include_bytes!("../images/test.bmp");



fn read_bmp(layer: &mut Layer<FramebufferArgb8888>, source : &[u8], pos_x : usize, pos_y : usize) {
    let w = source[18] as usize + 256 * source[19] as usize;
    let h = source[22] as usize + 256 * source[23] as usize;
    let offset = source[10] as usize;    
     for i in 0..h {
        for j in 0..w { 
            let col = Color{blue: source[3 * (j+(h-i-1)*w) + offset], green: source[3 * (j+(h-i-1)*w) + offset + 1], red: source[3 * (j+(h-i-1)*w) + offset + 2], alpha: 255};
            layer.print_point_color_at(j + pos_x, i + pos_y, col);
            //layer.print_point_color_at(j + pos_x, i + pos_y, bmp.color[(bmp.height - i - 1) * bmp.width + j]);
        }
    }


    //let mut col = [black; 32000];
    //for i in 0..(w * h) {
    //    col[i] = Color{blue: source[3 * i + offset], green: source[3 * i + offset + 1], red: source[3 * i + offset + 2], alpha: 255};
    //}
    //Bmp{width : w, height : h , color : col,}
    
}

//width must be a multiple of 4
fn draw_bmp(layer: &mut Layer<FramebufferArgb8888>, bmp : &Bmp, pos_x : usize, pos_y : usize) {
    for i in 0..bmp.height {
        for j in 0..bmp.width { 
            layer.print_point_color_at(j + pos_x, i + pos_y, bmp.color[(bmp.height - i - 1) * bmp.width + j]);
        }
    }
}



pub fn draw_image(layer: &mut Layer<FramebufferArgb8888>, img: &str, x_pos: usize, y_pos: usize) {
    if img == "example" {
        read_bmp(layer, EXAMPLE, x_pos, y_pos);
    }
    if img == "blitz" {
        read_bmp(layer, BLITZ, x_pos, y_pos);
    }
    if img == "blitz2" {
        read_bmp(layer, BLITZ2, x_pos, y_pos);
    }
    if img == "plants" {
        read_bmp(layer, PLANTS, x_pos, y_pos);
    }
    if img == "coal" {
        read_bmp(layer, COAL, x_pos, y_pos);
    }
     if img == "solar" {
        read_bmp(layer, SOLAR, x_pos, y_pos);
    }
    if img == "wind" {
        read_bmp(layer, WIND, x_pos, y_pos);
    }
    if img == "nuclear" {
        read_bmp(layer, NUCLEAR, x_pos, y_pos);
    }
    if img == "gas" {
        read_bmp(layer, GAS, x_pos, y_pos);
    }
    if img == "water" {
        read_bmp(layer, WATER, x_pos, y_pos);
    }
    if img == "back" {
        read_bmp(layer, BACK, x_pos, y_pos);
    }
    if img == "test" {
        read_bmp(layer, TEST, x_pos, y_pos);

    }
}