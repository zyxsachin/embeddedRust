#![warn(clippy::all)]

// Cost and then count
pub struct Powerplant {
    solar: (u32, u32, u32),
    wind: (u32, u32, u32),
    coal: (u32, u32, u32),
}

impl Powerplant {
    
    pub fn new() -> Self {
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

    pub fn add_wind(&mut self) {
        self.wind.0 *= 2;
        self.wind.1 += 1;
    }

    pub fn add_coal(&mut self) {
        self.coal.0 *= 2;
        self.coal.1 += 1;
    }
}