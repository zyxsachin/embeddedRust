#![warn(clippy::all)]

 // Cost, current number installed, power, co2 emissions
pub struct Event {
    event1: (u32, u32, u32, u32),
    event2: (u32, u32, u32, u32),
    event3: (u32, u32, u32, u32),
    event4: (u32, u32, u32, u32),
    event5: (u32, u32, u32, u32),
    event6: (u32, u32, u32, u32),
}

impl Event {
    
    pub fn new() -> Self {
        // Cost, current number installed, power, co2 emissions
        Event {
            event1: (10, 0, 1, 0),
            event2: (20, 0, 2, 0),
            event3: (100, 0, 20, 10),
            event4: (200, 0, 50, 30),
            event5: (10000, 0, 1000, 0),
            event6: (100_000, 0, 5000, 0),
        }
    }

  

    pub fn add_gas(&mut self) {
        self.gas.0 = (self.gas.0 as f32 * 1.25) as u32;
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

    pub fn get_gas_emissions(&mut self) -> u32{
        self.gas.1 * self.gas.3
    }

    pub fn set_gas(&mut self) {
        self.gas.0 = 100;
        self.gas.1 = 0;
    }



    pub fn get_watt(&mut self) -> u32 {
        self.get_solar_total_watt() + self.get_wind_total_watt() + self.get_gas_total_watt() + self.get_coal_total_watt() + self.get_nuclear_total_watt() + self.get_hydro_total_watt()
    }
    
    pub fn get_total_emissions(&mut self) -> u32 {
        self.get_gas_emissions() + self.get_coal_emissions()
    }
}