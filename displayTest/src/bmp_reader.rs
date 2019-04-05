
use stm32f7_discovery::{
    lcd::FramebufferAl88,
    lcd::FramebufferArgb8888,
    lcd::Lcd,
    lcd::Color,
    lcd::Layer,
    lcd::TextWriter,
};
use core::fmt::Write;



pub struct Bmp {
    width : usize,
    height : usize,
    color : [Color; 24300],
}

pub const EXAMPLE: &[u8] = include_bytes!("../images/e2.bmp");

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
pub fn draw_bmp(mut layer: &mut Layer<FramebufferArgb8888>, mut layer2: &mut Layer<FramebufferAl88>, bmp : &Bmp, pos_x : usize, pos_y : usize) {
    for i in 0..bmp.height {
        for j in 0..bmp.width { 
            layer.print_point_color_at(j + pos_x, i + pos_y, bmp.color[(bmp.height - i - 1) * bmp.width + j]);
        }
    }
    let mut text_layer = layer2.text_writer();
     text_layer.x_pos = 50;
     text_layer.y_pos = 50;
    for i in 0..16 {
             text_layer.write_fmt(format_args!("{},{},{}\n", bmp.color[i].red, bmp.color[i].green, bmp.color[i].blue));

    }

}

pub fn draw_example(mut layer: &mut Layer<FramebufferArgb8888>,mut layer2: &mut Layer<FramebufferAl88>) {
    let example = read_bmp(EXAMPLE);
    draw_bmp(layer, layer2, &example, 10, 20)
}