
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

static BLITZ: &[u8] = include_bytes!("../images/Blitz.bmp");
static BLITZ2: &[u8] = include_bytes!("../images/Blitz2.bmp");
static PLANTS: &[u8] = include_bytes!("../images/powerpflanzen.bmp");
static COAL: &[u8] = include_bytes!("../images/coal.bmp");
static SOLAR: &[u8] = include_bytes!("../images/sun.bmp");
static WATER: &[u8] = include_bytes!("../images/damm.bmp");
static NUCLEAR: &[u8] = include_bytes!("../images/nuke.bmp");
static GAS: &[u8] = include_bytes!("../images/gas.bmp");
static  WIND: &[u8] = include_bytes!("../images/wind.bmp");
static BACK: &[u8] = include_bytes!("../images/back.bmp");
static TREE: &[u8] = include_bytes!("../images/tree.bmp");
static BATTERY: &[u8] = include_bytes!("../images/battery.bmp");
static INFRASTRUCTURE: &[u8] = include_bytes!("../images/infrastructure.bmp");
static EUROPAPARK: &[u8] = include_bytes!("../images/europapark.bmp");
static DC: &[u8] = include_bytes!("../images/acdc.bmp");
static AC: &[u8] = include_bytes!("../images/hochspannungs.bmp");
//pub const TEST: &[u8] = include_bytes!("../images/test.bmp");
//pub const TEST2: &[u8] = include_bytes!("../images/test2.bmp");
static BACKGROUND: &[u8] = include_bytes!("../images/background.bmp"); 


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

fn draw_background(layer: &mut Layer<FramebufferArgb8888>, source : &[u8]) {
    let offset = source[10] as usize;
    let sky_blue = Color{red: 51, green: 204, blue: 255, alpha:255};
    let blue = Color{red: 0,green: 0 ,blue: 255,alpha: 255};
    
    let mut n = offset;
    for i in 0..272 {
        for j in 0..60 {
            if source[n] % 2 == 0 {
                layer.print_point_color_at(8*j+7, 271-i, blue);
            }
            if (source[n]/2) % 2 == 0 {
                layer.print_point_color_at(8*j+6, 271-i, blue);
            }
            if (source[n]/4) % 2 == 0 {
                layer.print_point_color_at(8*j+5, 271-i, blue);
            }
            if (source[n]/8) % 2 == 0 {
                layer.print_point_color_at(8*j+4, 271-i, blue);
            }
            if (source[n]/16) % 2 == 0 {
                layer.print_point_color_at(8*j+3, 271-i, blue);
            }
            if (source[n]/32) % 2 == 0 {
                layer.print_point_color_at(8*j+2, 271-i, blue);
            }
            if (source[n]/64) % 2 == 0 {
                layer.print_point_color_at(8*j+1, 271-i, blue);
            }
            if (source[n]/128) % 2 == 0 {
                layer.print_point_color_at(8*j,   271-i, blue);
            }
            n += 1;
        }
        
    }
}



pub fn draw_image(layer: &mut Layer<FramebufferArgb8888>, img: &str, x_pos: usize, y_pos: usize) {

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
    if img == "tree" {
        read_bmp(layer, TREE, x_pos, y_pos);
    }
    if img == "battery" {
        read_bmp(layer, BATTERY, x_pos, y_pos);
    }
    if img == "infrastructure" {
        read_bmp(layer, INFRASTRUCTURE, x_pos, y_pos);
    }
    if img == "europapark" {
        read_bmp(layer, EUROPAPARK, x_pos, y_pos);
    }
    if img == "ac" {
        read_bmp(layer, AC, x_pos, y_pos);
    }
    if img == "dc" {
        read_bmp(layer, DC, x_pos, y_pos);
    }
    if img == "back" {
        read_bmp(layer, BACK, x_pos, y_pos);
    }
    if img == "test" {
        //read_bmp(layer, TEST, x_pos, y_pos);
    }
    if img == "test2" {
        //read_bmp(layer, TEST2, x_pos, y_pos);
    }
    if img == "background" {
        draw_background(layer, BACKGROUND);
    }
}
