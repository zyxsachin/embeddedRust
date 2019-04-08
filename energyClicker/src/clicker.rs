#![warn(clippy::all)]

use crate::powerplant;
use crate::infrastructure;
use crate::carbon_dioxide;

pub struct Clicker {
    joule: u32,
    clicked: bool,
    j_per_click: u32,
    watt: u32, 
    last_touch_pos: (u16, u16),
    powerplant: powerplant::Powerplant,
    infrastructure: infrastructure::Infrastructure,
    carbon_dioxide: carbon_dioxide::GreenhouseGas,
    energy_demand: u32,
    new_energy_demand: u32,
    second_new_energy_demand: u32,
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
    
    pub fn new(pp: powerplant::Powerplant, is: infrastructure::Infrastructure, co: carbon_dioxide::GreenhouseGas) -> Self {
        Clicker {
            joule: 0,
            clicked: false,
            j_per_click: 1,
            watt: 0,          
            last_touch_pos: (0, 0),
            powerplant: pp,
            infrastructure: is,
            carbon_dioxide: co,
            energy_demand: 0,
            new_energy_demand: 1,
            second_new_energy_demand: 1,
        }
    }

    pub fn increment(&mut self) -> bool {
        
        // if self.energy_demand > self.powerplant.get_watt() {
        //     let demand = self.energy_demand - self.powerplant.get_watt();
        //     if demand > self.joule {
        //         return false;
        //     }
        // }
        
        self.joule += self.powerplant.get_watt();// - self.energy_demand;
        true
    }

    pub fn get_demand(&mut self) -> u32 {
        self.energy_demand
    }

    pub fn reset_clicks(&mut self) {
        self.last_touch_pos = (0, 0);
        self.clicked = false;
    }

    pub fn energy_tick(&mut self) {
        self.joule += self.j_per_click;
        self.clicked = true;
    }

    pub fn increase_demand(&mut self) {        
        let temp = self.energy_demand;
        self.energy_demand = self.new_energy_demand + self.second_new_energy_demand;
        self.new_energy_demand = self.second_new_energy_demand;
        self.second_new_energy_demand = temp;

    }

    pub fn reset_demand(&mut self) {
        self.energy_demand = 1;
        self.new_energy_demand = 1;
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

        let infrastructure_x = 330;
        let infrastructure_y = 80;

        let mode0_width = 100;
        let mode0_height = 100;


        if !self.clicked && dist(touch.0 as usize, touch.1 as usize, centre_x, centre_y) < radius {
//touch.x > 160 && touch.x < 320 && touch.y > 61 && touch.y < 211 {// && ticks - last_click > 50{
            if dist(touch.0 as usize, touch.1 as usize, self.last_touch_pos.0 as usize, self.last_touch_pos.1 as usize) > 10 {
                return (true, 0);
            }
            
        
            self.last_touch_pos = (touch.0, touch.1);            
        }

        else if !self.clicked &&  touch.0 > powerplant_x && touch.0 < powerplant_x + mode0_width 
                && touch.1 > powerplant_y && touch.1 < powerplant_y + mode0_height  {
            return (false, 1);
           
        }

        else if !self.clicked &&  touch.0 > infrastructure_x && touch.0 < infrastructure_x + mode0_width 
                && touch.1 > infrastructure_y && touch.1 < infrastructure_y + mode0_height {
            return (false, 2);
           
        }

        // draw::draw_rectangle(&mut layer_1, 50, 80, 100, 100, black);
        // draw::draw_rectangle(&mut layer_1, 330, 80, 100, 100, black);

        (false, 0)
    }

    // Returns a bool to check if there were changes and an int which indicates the mode
    pub fn check_mode1_clicked(&mut self, touch: (u16, u16)) -> (bool, i8) {

        let mode1_return_x = 0;
        let mode1_return_y = 0;

        let mode1_return_width = 100;
        let mode1_return_height = 272;

        let mode1_width = 100;
        let mode1_height = 100;
        
        let solar_x = self.powerplant.get_solar_coord().0 as u16;
        let solar_y = self.powerplant.get_solar_coord().1 as u16;

        let wind_x = self.powerplant.get_wind_coord().0 as u16;
        let wind_y = self.powerplant.get_wind_coord().1 as u16;

        let gas_x = self.powerplant.get_gas_coord().0 as u16;
        let gas_y = self.powerplant.get_gas_coord().1 as u16;

        let coal_x = self.powerplant.get_coal_coord().0 as u16;
        let coal_y = self.powerplant.get_coal_coord().1 as u16;

        let nuclear_x = self.powerplant.get_nuclear_coord().0 as u16;
        let nuclear_y = self.powerplant.get_nuclear_coord().1 as u16;

        let hydro_x = self.powerplant.get_hydro_coord().0 as u16;
        let hydro_y = self.powerplant.get_hydro_coord().1 as u16;

        if !self.clicked &&  touch.0 > mode1_return_x && touch.0 < mode1_return_x + mode1_return_width 
                && touch.1 > mode1_return_y && touch.1 < mode1_return_y + mode1_return_height {
            return (false, 0);          
        }

        else if !self.clicked &&  touch.0 > solar_x && touch.0 < solar_x + mode1_width 
                && touch.1 > solar_y && touch.1 < solar_y + mode1_height {
            if self.joule >= self.powerplant.get_solar_cost() {
                self.joule -= self.powerplant.get_solar_cost();
                self.powerplant.add_solar();
                self.clicked = true;
                return (true, 1);

            }
   
            return (false, 1);          
        }

        else if !self.clicked &&  touch.0 > wind_x && touch.0 < wind_x + mode1_width 
                && touch.1 > wind_y && touch.1 < wind_y + mode1_height {
            if self.joule >= self.powerplant.get_wind_cost() {
                self.joule -= self.powerplant.get_wind_cost();
                self.powerplant.add_wind();
                self.clicked = true;
                return (true, 1);

            }
            return (true, 1);          
        }

        else if !self.clicked &&  touch.0 > gas_x && touch.0 < gas_x + mode1_width 
                && touch.1 > gas_y && touch.1 < gas_y + mode1_height {
            if self.joule >= self.powerplant.get_gas_cost() {
                self.joule -= self.powerplant.get_gas_cost();
                self.powerplant.add_gas();
                self.clicked = true;
                return (true, 1);
            }
            return (true, 1);          
        }

        else if !self.clicked &&  touch.0 > coal_x && touch.0 < coal_x + mode1_width 
                && touch.1 > coal_y && touch.1 < coal_y + mode1_height {
            if self.joule >= self.powerplant.get_coal_cost() {
                self.joule -= self.powerplant.get_coal_cost();
                self.powerplant.add_coal();
                self.clicked = true;
                return (true, 1);

            }
            return (true, 1);          
        }

        else if !self.clicked &&  touch.0 > nuclear_x && touch.0 < nuclear_x + mode1_width 
                && touch.1 > nuclear_y && touch.1 < nuclear_y + mode1_height {
            if self.joule >= self.powerplant.get_nuclear_cost() {
                self.joule -= self.powerplant.get_nuclear_cost();
                self.powerplant.add_nuclear();
                self.clicked = true;
                return (true, 1);

            }
            return (true, 1);          
        }


        else if !self.clicked &&  touch.0 > hydro_x && touch.0 < hydro_x + mode1_width 
                && touch.1 > hydro_y && touch.1 < hydro_y + mode1_height {
            if self.joule >= self.powerplant.get_hydro_cost() {
                self.joule -= self.powerplant.get_hydro_cost();
                self.powerplant.add_hydro();
                self.clicked = true;
                return (true, 1);

            }
            return (true, 1);          
        }

        (false, 1)
    }

    // Returns a bool to check if there were changes and an int which indicates the mode
    pub fn check_mode2_clicked(&mut self, touch: (u16, u16)) -> (bool, i8) {

        let mode2_return_x = 0;
        let mode2_return_y = 0;

        let mode2_return_width = 100;
        let mode2_return_height = 272;

        let mode2_width = 100;
        let mode2_height = 100;
        
        let battery_x = self.infrastructure.get_battery_coord().0 as u16;
        let battery_y = self.infrastructure.get_battery_coord().1 as u16;

        let smart_grid_x = self.infrastructure.get_smart_grid_coord().0 as u16;
        let smart_grid_y = self.infrastructure.get_smart_grid_coord().1 as u16;

        let hvac_x = self.infrastructure.get_hvac_coord().0 as u16;
        let hvac_y = self.infrastructure.get_hvac_coord().1 as u16;

        let hvdc_x = self.infrastructure.get_hvdc_coord().0 as u16;
        let hvdc_y = self.infrastructure.get_hvdc_coord().1 as u16;

        let tree_x = self.carbon_dioxide.get_tree_coord().0 as u16;
        let tree_y = self.carbon_dioxide.get_tree_coord().1 as u16;

        let special_x = self.carbon_dioxide.get_special_coord().0 as u16;
        let special_y = self.carbon_dioxide.get_special_coord().1 as u16;

        if !self.clicked &&  touch.0 > mode2_return_x && touch.0 < mode2_return_x + mode2_return_width 
                && touch.1 > mode2_return_y && touch.1 < mode2_return_y + mode2_return_height {
            return (false, 0);          
        }

        else if !self.clicked &&  touch.0 > battery_x && touch.0 < battery_x + mode2_width 
                && touch.1 > battery_y && touch.1 < battery_y + mode2_height {
            if self.joule >= self.infrastructure.get_battery_cost() {
                self.joule -= self.infrastructure.get_battery_cost();
                self.infrastructure.add_battery();
                self.clicked = true;
                return (true, 2);

            }
   
            return (false, 2);          
        }

        else if !self.clicked &&  touch.0 > smart_grid_x && touch.0 < smart_grid_x + mode2_width 
                && touch.1 > smart_grid_y && touch.1 < smart_grid_y + mode2_height {
            if self.joule >= self.infrastructure.get_smart_grid_cost() {
                self.joule -= self.infrastructure.get_smart_grid_cost();
                self.infrastructure.add_smart_grid();
                self.clicked = true;
                return (true, 2);

            }
   
            return (false, 2);          
        }

        else if !self.clicked &&  touch.0 > hvac_x && touch.0 < hvac_x + mode2_width 
                && touch.1 > hvac_y && touch.1 < hvac_y + mode2_height {
            if self.joule >= self.infrastructure.get_hvac_cost() {
                self.joule -= self.infrastructure.get_hvac_cost();
                self.infrastructure.add_hvac();
                self.clicked = true;
                return (true, 2);

            }
   
            return (false, 2);          
        }

        else if !self.clicked &&  touch.0 > hvdc_x && touch.0 < hvdc_x + mode2_width 
                && touch.1 > hvdc_y && touch.1 < hvdc_y + mode2_height {
            if self.joule >= self.infrastructure.get_hvdc_cost() {
                self.joule -= self.infrastructure.get_hvdc_cost();
                self.infrastructure.add_hvdc();
                self.clicked = true;
                return (true, 2);

            }
   
            return (false, 2);          
        }

        else if !self.clicked &&  touch.0 > tree_x && touch.0 < tree_x + mode2_width 
                && touch.1 > tree_y && touch.1 < tree_y + mode2_height {
            if self.joule >= self.carbon_dioxide.get_tree_cost() {
                self.joule -= self.carbon_dioxide.get_tree_cost();
                self.carbon_dioxide.add_tree();
                self.clicked = true;
                return (true, 2);

            }
   
            return (false, 2);          
        }

        else if !self.clicked &&  touch.0 > special_x && touch.0 < special_x + mode2_width 
                && touch.1 > special_y && touch.1 < special_y + mode2_height {
            if self.joule >= self.carbon_dioxide.get_special_cost() {
                self.joule -= self.carbon_dioxide.get_special_cost();
                self.carbon_dioxide.add_special();
                self.clicked = true;
                return (true, 2);

            }
   
            return (false, 2);          
        }


        (false, 2)
    }

    pub fn update_watt(&mut self) {
        self.watt = self.powerplant.get_watt()
    }

    pub fn update_joule_per_click(&mut self) {
        self.j_per_click = self.infrastructure.get_joule_per_click()
    }

    pub fn get_joule_per_click(&mut self) -> u32 {
        self.j_per_click
    }

    pub fn get_joule(&mut self) -> u32 {
        self.joule
    }

    pub fn get_watt(&mut self) -> u32 {
        self.watt
    }

    pub fn get_emissions(&mut self) -> u32 {
        self.powerplant.get_total_emissions()
    }

    pub fn get_co2_absorb(&mut self) -> u32 {
        self.carbon_dioxide.get_co2_absorb()
    }

 
    pub fn get_solar(&mut self) -> (u32, u32, u32) {
        self.powerplant.get_solar()
    }

    pub fn get_wind(&mut self) -> (u32, u32, u32) {
        self.powerplant.get_wind()
    }
    
    pub fn get_gas(&mut self) -> (u32, u32, u32) {
        self.powerplant.get_gas()
    }

    pub fn get_coal(&mut self) -> (u32, u32, u32) {
        self.powerplant.get_coal()
    }

    pub fn get_nuclear(&mut self) -> (u32, u32, u32) {
        self.powerplant.get_nuclear()
    }

    pub fn get_hydro(&mut self) -> (u32, u32, u32) {
        self.powerplant.get_hydro()
    }

    pub fn get_battery(&mut self) -> (u32, u32, u32) {
        self.infrastructure.get_battery()
    }

    pub fn get_smart_grid(&mut self) -> (u32, u32, u32) {
        self.infrastructure.get_smart_grid()
    }

    pub fn get_hvac(&mut self) -> (u32, u32, u32) {
        self.infrastructure.get_hvac()
    }

    pub fn get_hvdc(&mut self) -> (u32, u32, u32) {
        self.infrastructure.get_hvdc()
    }

    pub fn get_tree(&mut self) -> (u32, u32, u32) {
        self.carbon_dioxide.get_tree()
    }

    pub fn get_special(&mut self) -> (u32, u32, u32) {
        self.carbon_dioxide.get_special()
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


