#![warn(clippy::all)]

// Cost and then count
pub struct Powerplant {
    solar: (u32, u32, u32),
    wind: (u32, u32, u32),
    gas: (u32, u32, u32),
    coal: (u32, u32, u32),
    nuclear: (u32, u32, u32),
    hydro: (u32, u32, u32),
}

impl Powerplant {
    
    pub fn new() -> Self {
        // Cost, current number instealled, power
        Powerplant {
            solar: (10, 0, 1),
            wind: (50, 0, 5),
            gas: (100, 0, 25),
            coal: (500, 0, 100),
            nuclear: (10000, 0, 1000),
            hydro: (100_000, 0, 5000),
        }
    }

    pub fn add_solar(&mut self) {
        self.solar.0 *= 2;
        self.solar.1 += 1;
    }

    pub fn get_solar_total_watt(&mut self) -> u32 {
        self.solar.1 * self.solar.2
    }

    pub fn get_solar_cost(&mut self) -> u32 {
        self.solar.0
    }

    pub fn get_solar(&mut self) -> (u32, u32, u32) {
        (self.solar.0, self.solar.1, self.solar.2)
    }

    pub fn get_solar_coord(&mut self) -> (u32, u32) {
        (120, 5)
    }

    pub fn add_wind(&mut self) {
        self.wind.0 *= 2;
        self.wind.1 += 1;
    }

    pub fn get_wind_total_watt(&mut self) -> u32 {
        self.wind.1 * self.wind.2
    }

    pub fn get_wind_cost(&mut self) -> u32 {
        self.wind.0
    }

    pub fn get_wind(&mut self) -> (u32, u32, u32) {
        (self.wind.0, self.wind.1, self.wind.2)
    }

    pub fn get_wind_coord(&mut self) -> (u32, u32) {
        (240, 5)
    }

    pub fn add_gas(&mut self) {
        self.gas.0 *= 2;
        self.gas.1 += 1;
    }

    pub fn get_gas_total_watt(&mut self) -> u32{
        self.gas.1 * self.gas.2
    }

    pub fn get_gas_cost(&mut self) -> u32 {
        self.gas.0
    }

    pub fn get_gas(&mut self) -> (u32, u32, u32) {
        (self.gas.0, self.gas.1, self.gas.2)
    }
 
    pub fn get_gas_coord(&mut self) -> (u32, u32) {
        (360, 5)
    }

    pub fn add_coal(&mut self) {
        self.coal.0 *= 2;
        self.coal.1 += 1;
    }

    pub fn get_coal_total_watt(&mut self) -> u32 {
        self.coal.1 * self.coal.2
    }

    pub fn get_coal_cost(&mut self) -> u32 {
        self.coal.0
    }

    pub fn get_coal(&mut self) -> (u32, u32, u32) {
        (self.coal.0, self.coal.1, self.coal.2)
    }

    pub fn get_coal_coord(&mut self) -> (u32, u32) {
        (120, 135)
    }


    pub fn add_nuclear(&mut self) {
        self.nuclear.0 *= 2;
        self.nuclear.1 += 1;
    }

    pub fn get_nuclear_total_watt(&mut self) -> u32 {
        self.nuclear.1 * self.nuclear.2
    }

    pub fn get_nuclear_cost(&mut self) -> u32 {
        self.nuclear.0
    }

    pub fn get_nuclear(&mut self) -> (u32, u32, u32) {
        (self.nuclear.0, self.nuclear.1, self.nuclear.2)
    }

    pub fn get_nuclear_coord(&mut self) -> (u32, u32) {
        (240, 135)
    }

    pub fn add_hydro(&mut self) {
        self.hydro.0 *= 2;
        self.hydro.1 += 1;
    }

    pub fn get_hydro_total_watt(&mut self) -> u32 {
        self.hydro.1 * self.hydro.2
    }

    pub fn get_hydro_cost(&mut self) -> u32 {
        self.hydro.0
    }

    pub fn get_hydro(&mut self) -> (u32, u32, u32) {
        (self.hydro.0, self.hydro.1, self.hydro.2)
    }

    pub fn get_hydro_coord(&mut self) -> (u32, u32) {
        (360, 135)
    }

    pub fn get_watt(&mut self) -> u32 {
        self.get_solar_total_watt() + self.get_wind_total_watt() + self.get_gas_total_watt() + self.get_coal_total_watt() + self.get_nuclear_total_watt()
    }
}