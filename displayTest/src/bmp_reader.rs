
use stm32f7_discovery::{
    lcd::FramebufferAl88,
    lcd::FramebufferArgb8888,
    lcd::Lcd,
    lcd::Color,
    lcd::Layer,
    lcd::TextWriter,
};

pub struct Bmp {
    width : usize,
    height : usize,
    color : [Color; 24300],
}

pub const EXAMPLE: &[u8] = include_bytes!("../images/example.bmp");

pub fn read_bmp(source : &[u8]) -> Bmp {
    let w = source[18] as usize;
    let h = source[22] as usize;
    let offset = source[10] as usize;
    let black = Color{red: 0,green: 0 ,blue: 0,alpha: 255};
    let mut col = [black; 24300];
    for i in 0..(w * h) {
        col[i] = Color{red: source[3 * i + offset] ,green: source[3 * i + offset + 1] ,blue: source[3 * i + offset + 2],alpha: 255};
    }
    Bmp{width : w, height : h , color : col,}

}

pub fn draw_bmp(mut layer: &mut Layer<FramebufferArgb8888>, bmp : &Bmp, pos_x : usize, pos_y : usize) {
    for i in 0..bmp.height+1 {
        for j in 0..bmp.width+1 {
            layer.print_point_color_at(j + pos_x, i + pos_y, bmp.color[j + i * bmp.width]);
        }
    }
}

pub fn draw_example(mut layer: &mut Layer<FramebufferArgb8888>) {
    let example = read_bmp(EXAMPLE);
    draw_bmp(layer, &example, 10, 20)
}