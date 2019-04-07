#![warn(clippy::all)]

// Cost and then count
pub struct Infrastructure {
    battery: (u32, u32, u32),
    smart_grid: (u32, u32, u32),
    hvac: (u32, u32, u32),
    hvdc: (u32, u32, u32),
}

impl Infrastructure {
    
    pub fn new() -> Self {
        // Cost, current number installed, joules per click
        Infrastructure {
            battery: (20, 0, 1),
            smart_grid: (50, 0, 2),
            hvac: (500, 0, 5),
            hvdc: (5000, 0, 10),
        }
    }

    pub fn add_battery(&mut self) {
        self.battery.0 *= 2;
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
        self.smart_grid.0 *= 2;
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
        self.hvac.0 *= 2;
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
        self.hvdc.0 *= 2;
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

    pub fn get_joule_per_click(&mut self) -> u32 {
        1 + self.get_battery_total_click() + self.get_smart_grid_total_click() + self.get_hvac_total_click() + self.get_hvdc_total_click()
    }

}