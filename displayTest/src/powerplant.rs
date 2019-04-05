#![warn(clippy::all)]

// Cost and then count
pub struct Powerplant {
    solar: (u32, u32, u32),
    wind: (u32, u32, u32),
    coal: (u32, u32, u32),
}

impl Powerplant {
    
    pub fn new() -> Self {
        // Cost, current number instealled, power
        Powerplant {
            solar: (10, 0, 1),
            wind: (50, 0, 5),
            coal: (100, 0, 25),
        }
    }

    pub fn add_solar(&mut self) {
        self.solar.0 *= 2;
        self.solar.1 += 1;
    }

    pub fn get_solar_watt(&mut self) -> u32 {
        self.solar.1 * self.solar.2
    }

    pub fn get_solar_cost(&mut self) -> u32 {
        self.solar.0
    }

    pub fn get_solar(&mut self) -> (u32, u32, u32) {
        (self.solar.0, self.solar.1, self.solar.2)
    }

    pub fn add_wind(&mut self) {
        self.wind.0 *= 2;
        self.wind.1 += 1;
    }

    pub fn get_wind_watt(&mut self) -> u32 {
        self.wind.1 * self.wind.2
    }

    pub fn get_wind_cost(&mut self) -> u32 {
        self.wind.0
    }

    pub fn get_wind(&mut self) -> (u32, u32, u32) {
        (self.wind.0, self.wind.1, self.wind.2)
    }

    pub fn add_coal(&mut self) {
        self.coal.0 *= 2;
        self.coal.1 += 1;
    }

    pub fn get_coal_watt(&mut self) -> u32 {
        self.coal.1 * self.coal.2
    }

    pub fn get_coal_cost(&mut self) -> u32 {
        self.coal.0
    }

    pub fn get_coal(&mut self) -> (u32, u32, u32) {
        (self.coal.0, self.coal.1, self.coal.2)
    }

    pub fn get_watt(&mut self) -> u32 {
        self.get_solar_watt() + self.get_wind_watt() + self.get_coal_watt()
    }
}