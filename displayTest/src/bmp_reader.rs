
use stm32f7_discovery::{
    lcd::FramebufferArgb8888,
    lcd::Color,
    lcd::Layer,
};



pub struct Bmp {
    width : usize,
    height : usize,
    color : [Color; 24300],
}

pub const EXAMPLE: &[u8] = include_bytes!("../images/e2.bmp");
pub const BLITZ: &[u8] = include_bytes!("../images/Blitz.bmp");
pub const BLITZ2: &[u8] = include_bytes!("../images/Blitz2.bmp");
pub const PLANTS: &[u8] = include_bytes!("../images/powerpflanzen.bmp");
pub const COAL: &[u8] = include_bytes!("../images/coal.bmp");
pub const SOLAR: &[u8] = include_bytes!("../images/sun.bmp");
pub const WIND: &[u8] = include_bytes!("../images/wind.bmp");




pub fn read_bmp(source : &[u8]) -> Bmp {
    let w = source[18] as usize;
    let h = source[22] as usize;
    let offset = source[10] as usize;
    let black = Color{red: 255,green: 0 ,blue: 0,alpha: 255};
    let mut col = [black; 24300];
    for i in 0..(w * h) {
        col[i] = Color{blue: source[3 * i + offset], green: source[3 * i + offset + 1], red: source[3 * i + offset + 2],alpha: 255};
    }
    Bmp{width : w, height : h , color : col,}

}

//width must be a multiple of 4
pub fn draw_bmp(layer: &mut Layer<FramebufferArgb8888>, bmp : &Bmp, pos_x : usize, pos_y : usize) {
    for i in 0..bmp.height {
        for j in 0..bmp.width { 
            layer.print_point_color_at(j + pos_x, i + pos_y, bmp.color[(bmp.height - i - 1) * bmp.width + j]);
        }
    }
}

pub fn draw_image(layer: &mut Layer<FramebufferArgb8888>, img: &str, x_pos: usize, y_pos: usize) {
    if img == "example" {
        let bmp = read_bmp(EXAMPLE);
        draw_bmp(layer, &bmp, x_pos, y_pos);
    }
    if img == "blitz" {
        let bmp = read_bmp(BLITZ);
        draw_bmp(layer, &bmp, x_pos, y_pos);
    }
    if img == "blitz2" {
        let bmp = read_bmp(BLITZ2);
        draw_bmp(layer, &bmp, x_pos, y_pos);
    }
    if img == "plants" {
        let bmp = read_bmp(PLANTS);
        draw_bmp(layer, &bmp, x_pos, y_pos);
    }
    if img == "coal" {
        let bmp = read_bmp(COAL);
        draw_bmp(layer, &bmp, x_pos, y_pos);
    }
    if img == "solar" {
        let bmp = read_bmp(SOLAR);
        draw_bmp(layer, &bmp, x_pos, y_pos);
    }
    if img == "wind" {
        let bmp = read_bmp(WIND);
        draw_bmp(layer, &bmp, x_pos, y_pos);
    }
}