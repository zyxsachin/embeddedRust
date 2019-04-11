#![warn(clippy::all)]

// Cost and then count
pub struct Infrastructure {
    battery: (u32, u32, u32),
    smart_grid: (u32, u32, u32),
    hvac: (u32, u32, u32),
    hvdc: (u32, u32, u32),
    special: (u32, u32, u32),
}

impl Infrastructure {
    
    pub fn new() -> Self {
        // Cost, current number installed, joules per click
        Infrastructure {
            battery: (20, 0, 1),
            smart_grid: (50, 0, 2),
            hvac: (500, 0, 5),
            hvdc: (5000, 0, 10),
            special: (100_000, 0, 100_000),
        }
    }

    pub fn add_battery(&mut self) {
        self.battery.0 = (self.battery.0 as f32 * 1.1) as u32;;
        self.battery.1 += 1;
    }

    pub fn get_battery_total_click(&mut self) -> u32 {
        self.battery.1 * self.battery.2
    }

    pub fn get_battery_cost(&mut self) -> u32 {
        self.battery.0
    }

    pub fn get_battery(&mut self) -> (u32, u32, u32) {
        (self.battery.0, self.battery.1, self.battery.2)
    }

    pub fn get_battery_coord(&mut self) -> (u32, u32) {
        (120, 5)
    }


    pub fn add_smart_grid(&mut self) {
        self.smart_grid.0 = (self.smart_grid.0 as f32 * 1.2) as u32;
        self.smart_grid.1 += 1;
    }

    pub fn get_smart_grid_total_click(&mut self) -> u32 {
        self.smart_grid.1 * self.smart_grid.2
    }

    pub fn get_smart_grid_cost(&mut self) -> u32 {
        self.smart_grid.0
    }

    pub fn get_smart_grid(&mut self) -> (u32, u32, u32) {
        (self.smart_grid.0, self.smart_grid.1, self.smart_grid.2)
    }

    pub fn get_smart_grid_coord(&mut self) -> (u32, u32) {
        (240, 5)
    }


    pub fn add_hvac(&mut self) {
        self.hvac.0 = (self.hvac.0 as f32 * 1.3) as u32;
        self.hvac.1 += 1;
    }

    pub fn get_hvac_total_click(&mut self) -> u32 {
        self.hvac.1 * self.hvac.2
    }

    pub fn get_hvac_cost(&mut self) -> u32 {
        self.hvac.0
    }

    pub fn get_hvac(&mut self) -> (u32, u32, u32) {
        (self.hvac.0, self.hvac.1, self.hvac.2)
    }

    pub fn get_hvac_coord(&mut self) -> (u32, u32) {
        (360, 5)
    }


    pub fn add_hvdc(&mut self) {
        self.hvdc.0 = (self.hvdc.0 as f32 * 1.4) as u32;
        self.hvdc.1 += 1;
    }

    pub fn get_hvdc_total_click(&mut self) -> u32 {
        self.hvdc.1 * self.hvdc.2
    }

    pub fn get_hvdc_cost(&mut self) -> u32 {
        self.hvdc.0
    }

    pub fn get_hvdc(&mut self) -> (u32, u32, u32) {
        (self.hvdc.0, self.hvdc.1, self.hvdc.2)
    }

    pub fn get_hvdc_coord(&mut self) -> (u32, u32) {
        (120, 135)
    }


    pub fn add_special(&mut self) {
        // if self.special.0 >= 10_000_000 {
            self.special.0 = 1_000_000;
        // }
        // else {
        //     self.special.0 *= 10;
        // }
       
        self.special.1 += 1;
    }

    pub fn get_special_click(&mut self) -> u32 {
        self.special.1 * self.special.2
    }

    pub fn get_special_cost(&mut self) -> u32 {
        self.special.0
    }

    pub fn get_special(&mut self) -> (u32, u32, u32) {
        (self.special.0, self.special.1, self.special.2)
    }

    pub fn get_special_coord(&mut self) -> (u32, u32) {
        (360, 135)
    }

    pub fn get_joule_per_click(&mut self) -> u32 {
        1 + self.get_battery_total_click() + self.get_smart_grid_total_click() + self.get_hvac_total_click() + self.get_hvdc_total_click() + self.get_special_click()
    }

}