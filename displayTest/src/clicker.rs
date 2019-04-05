#![warn(clippy::all)]

use crate::powerplant;

pub struct Clicker {
    joule: u32,
    clicked: bool,
    j_per_click: u32,
    watt: u32, 
    last_touch_pos: (u16, u16),
    powerplant: powerplant::Powerplant,
}

    // let mut joule: u32 = 0;
    // let mut last_led_toggle = system_clock::ticks();

    
    // let mut clicked = 0;
    // let mut clicked_ticks = 0;
    // let mut clicker_color = sky_blue;

    // let mut j_per_click = 1;

    // let mut watt = 0;

    // let mut last_click = 0;
    // let mut last_touch_pos: (u16, u16) = (0, 0);

impl Clicker {
    
    pub fn new(pp: powerplant::Powerplant) -> Self {
        Clicker {
            joule: 0,
            clicked: false,
            j_per_click: 1,
            watt: 0,          
            last_touch_pos: (0, 0),
            powerplant: pp,
        }
    }

    pub fn increment(&mut self) {
         self.joule += self.watt;
    }

    pub fn reset_clicks(&mut self) {
        self.last_touch_pos = (0, 0);
        self.clicked = false;
    }

    pub fn energy_tick(&mut self) {
        self.joule += self.j_per_click;
        self.clicked = true;
    }

    // Returns a bool to check if the central clicker was clicked and an int which indicates the mode
    pub fn check_mode0_clicked(&mut self, touch: (u16, u16)) -> (bool, i8) { 
        
        let max_x = 480;
        let max_y = 272;
        let centre_x = max_x / 2;
        let centre_y = max_y / 2;
        let radius = 50;

        let powerplant_x = 50;


        let powerplant_y = 80;

        let mode0_width = 100;
        let mode0_height = 100;

        if !self.clicked && dist(touch.0 as usize, touch.1 as usize, centre_x, centre_y) < radius {
//touch.x > 160 && touch.x < 320 && touch.y > 61 && touch.y < 211 {// && ticks - last_click > 50{
            if dist(touch.0 as usize, touch.1 as usize, self.last_touch_pos.0 as usize, self.last_touch_pos.1 as usize) > 10 {
                return (true, 0);
            }
            
        
            self.last_touch_pos = (touch.0, touch.1);            
        }

        else if !self.clicked && touch.1 > powerplant_y && touch.1 < powerplant_y + mode0_height 
            &&  touch.0 > powerplant_x && touch.0 < powerplant_x + mode0_width {
            return (false, 1);
           
        }

        // draw::draw_rectangle(&mut layer_1, 50, 80, 100, 100, black);
        // draw::draw_rectangle(&mut layer_1, 330, 80, 100, 100, black);



        (false, 0)
    }

    // Returns a bool to check if there were changes and an int which indicates the mode
    pub fn check_mode1_clicked(&mut self, touch: (u16, u16)) -> (bool, i8) {
        (false, 1)
    }

    pub fn get_joule(&mut self) -> u32 {
        self.joule
    }

    pub fn get_watt(&mut self) -> u32 {
        self.watt
    }
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


