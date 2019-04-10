#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![warn(clippy::all)]
#![feature(alloc)]

//#[macro_use]
extern crate alloc;

mod clicker;
mod draw;
mod powerplant;
mod infrastructure;
mod bmp_reader;
mod carbon_dioxide;

// use alloc::sync::Arc;
// use alloc::vec::Vec;
// use stm32f7_discovery::future_mutex;



//use alloc::fmt as fmt;
//use core::fmt::Write;

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout as AllocLayout;
use core::panic::PanicInfo;
use cortex_m_rt::{entry, exception};
use stm32f7::stm32f7x6::{CorePeripherals, Peripherals};
use stm32f7_discovery::{
    gpio::{GpioPort, OutputPin},
    init,
    random::Rng,
    system_clock::{self, Hz},  touch, lcd::Color, i2c,
};


// static blue: Color = Color{red: 0,green: 0 ,blue: 255,alpha: 255};
// static green: Color = Color{red: 0,green: 255 ,blue: 0,alpha: 255};
// static black: Color = Color{red: 0,green: 0 ,blue: 0,alpha: 255};
// static white: Color = Color{red: 255,green: 255 ,blue: 255,alpha: 255};
// static grey: Color = Color{red: 127,green: 127 ,blue: 127,alpha: 127};
// static yellow: Color = Color{red: 255, green: 255, blue: 0, alpha: 255};
// static red: Color = Color{red: 255, green: 0, blue: 0, alpha: 255};

#[entry]
fn main() -> ! {
    let core_peripherals = CorePeripherals::take().unwrap();
    let mut systick = core_peripherals.SYST;

    let peripherals = Peripherals::take().unwrap();
    let mut rcc = peripherals.RCC;
    let mut pwr = peripherals.PWR;
    let mut flash = peripherals.FLASH;



    init::init_system_clock_216mhz(&mut rcc, &mut pwr, &mut flash);
    init::enable_gpio_ports(&mut rcc);

    let mut fmc = peripherals.FMC;
    let mut ltdc = peripherals.LTDC;
    let mut rng = peripherals.RNG;
    // let mut sai_2 = peripherals.SAI2;
    // let mut rng = peripherals.RNG;
    // let mut sdmmc = peripherals.SDMMC1;
    // let mut syscfg = peripherals.SYSCFG;
    // let mut ethernet_mac = peripherals.ETHERNET_MAC;
    // let mut ethernet_dma = peripherals.ETHERNET_DMA;

    let i2c_3 = peripherals.I2C3;

    let gpio_a = GpioPort::new(peripherals.GPIOA);
    let gpio_b = GpioPort::new(peripherals.GPIOB);
    let gpio_c = GpioPort::new(peripherals.GPIOC);
    let gpio_d = GpioPort::new(peripherals.GPIOD);
    let gpio_e = GpioPort::new(peripherals.GPIOE);
    let gpio_f = GpioPort::new(peripherals.GPIOF);
    let gpio_g = GpioPort::new(peripherals.GPIOG);
    let gpio_h = GpioPort::new(peripherals.GPIOH);
    let gpio_i = GpioPort::new(peripherals.GPIOI);
    let gpio_j = GpioPort::new(peripherals.GPIOJ);
    let gpio_k = GpioPort::new(peripherals.GPIOK);

    let mut pins = init::pins(
        gpio_a, gpio_b, gpio_c, gpio_d, gpio_e, gpio_f, gpio_g, gpio_h, gpio_i, gpio_j, gpio_k,
    );

    // let pp = powerplant::Powerplant::new();
    // let is = infrastructure::Infrastructure::new();
    let mut clicker = clicker::Clicker::new(powerplant::Powerplant::new(), infrastructure::Infrastructure::new(), carbon_dioxide::Carbondioxide::new());

     // i2c
    //i2c::init_pins_and_clocks(rcc, &mut gpio);
    let mut i2c_3 = i2c::init(i2c_3, &mut rcc);
    touch::check_family_id(&mut i2c_3).unwrap();

    let hertz = 100;

    // configure the systick timer 20Hz (20 ticks per second)
    init::init_systick(Hz(hertz), &mut systick, &rcc);
    systick.enable_interrupt();


    init::init_sdram(&mut rcc, &mut fmc);
    let mut lcd = init::init_lcd(&mut ltdc, &mut rcc);
    
    pins.display_enable.set(true);
    pins.backlight.set(true);

    let mut layer_1 = lcd.layer_1().unwrap();
    let mut layer_2 = lcd.layer_2().unwrap();
/*    
    let bg_color = Color{red: 255,green: 0 ,blue: 0,alpha: 255};
    let blue = Color{red: 0,green: 0 ,blue: 255,alpha: 255};
    let green = Color{red: 0,green: 255 ,blue: 0,alpha: 255};
    let black = Color{red: 0,green: 0 ,blue: 0,alpha: 255};
    let white = Color{red: 255,green: 255 ,blue: 255,alpha: 255};
    let grey = Color{red: 127,green: 127 ,blue: 127,alpha: 127};
    let yellow = Color{red: 255, green: 255, blue: 0, alpha: 255};
    let red = Color{red: 255, green: 0, blue: 0, alpha: 255};
*/
    let sky_blue = Color{red: 51, green: 204, blue: 255, alpha:255};
    layer_1.clear();
    layer_2.clear();
    lcd.set_background_color(sky_blue);


    // turn led on
    pins.led.set(true);
    // let mut last_led_toggle = system_clock::ticks();

    
    // let mut clicked_ticks = 0;
    // let mut clicker_color = sky_blue;
      

    // let max_x = 480;
    // let max_y = 272;
    // let centre_x = max_x / 2;
    // let centre_y = max_y / 2;
    // let radius = 50;

    let mut mode = 0;
    let mut mode_just_set = true;

    let mut circle_reset = false;

        
    let mut emissions: usize = 0;
    let max_emissions: usize = 450;

    draw::draw_mode0(&mut layer_1, &mut layer_2, max_emissions);


    let mut old_tick = system_clock::ticks();
    // let mut demand_tick = system_clock::ticks();

    let mut time = 0;

    let mut rng = Rng::init(&mut rng, &mut rcc).expect("RNG init failed");
   
    loop {
        let ticks = system_clock::ticks();

        if mode == 0 {
            if touch::touches(&mut i2c_3).unwrap().len() == 0 && circle_reset {
                //draw::color_circle(&mut layer_1, centre_x, centre_y, radius, sky_blue);
                bmp_reader::draw_image(&mut layer_1,"blitz", 190, 86);
                circle_reset = false;
            }
            if mode_just_set {
                if touch::touches(&mut i2c_3).unwrap().len() != 0 {
                    continue;
                }
                layer_1.clear();
                layer_2.clear();
                draw::draw_mode0(&mut layer_1, &mut layer_2, max_emissions);
                bmp_reader::draw_image(&mut layer_1,"blitz", 190, 86);
                mode_just_set = false;
            }
            if ticks - old_tick >= 100 {

                old_tick = ticks;
                clicker.increment();
                // let game_continue = clicker.increment();
                // if !game_continue {
                //     clicker = clicker::Clicker::new(powerplant::Powerplant::new(), infrastructure::Infrastructure::new());
                //     clicker.reset_demand();
                //     layer_1.clear();
                //     layer_2.clear();
                //     draw::draw_mode0(&mut layer_1, &mut layer_2);
                    
                // }
                time += 1;
                if time % 2 == 0 {
                    let random = rng.poll_and_get().expect("Failed to generate random number")%2;
                    if random == 0 {
                        mode = 4;
                        mode_just_set = true;
                        continue;
                    }
                }


                let current_emissions = emissions;
                emissions += clicker.get_emissions() as usize;
                let absorb = clicker.get_co2_absorb() as usize;
                if absorb > emissions {
                    emissions = 0;
                }
                else {
                    emissions -= absorb;
                }
                if i128::from(clicker.get_emissions()) - i128::from(clicker.get_co2_absorb()) > 50 {
                    emissions = current_emissions + 50;
                }
                if emissions > max_emissions {
                    mode = 3;
                    mode_just_set = true;
                    continue;
                }
                
                draw::draw_emissions(&mut layer_1, emissions, max_emissions);

                
            }

            // if ticks - demand_tick >= 500 {
            //     demand_tick = ticks;
            //     clicker.increase_demand();
            // }

            if touch::touches(&mut i2c_3).unwrap().len() == 0 {
                clicker.reset_clicks();             
            }

            else if touch::touches(&mut i2c_3).unwrap().len() == 1 {    
                for touch in &touch::touches(&mut i2c_3).unwrap() {  
                    let check_clicked = clicker.check_mode0_clicked((touch.x, touch.y));    
                    mode = check_clicked.1;
                    if check_clicked.0 {
                        clicker.energy_tick();
                        bmp_reader::draw_image(&mut layer_1,"blitz2", 190, 86);
                        //draw::color_circle(&mut layer_1, centre_x, centre_y, radius, yellow);
                        circle_reset = true;
                    }
                }
            }
 
            let joule = clicker.get_joule() as u32;
            let watt = clicker.get_watt();
            let j_per_click = clicker.get_joule_per_click();
            // let demand = clicker.get_demand();
            // let line1_start = "      Buy Powerplant     ";
            // let line1_end = "    Buy Infrastructure";
            // let offset =      "                         ";
            draw::write_string(&mut layer_2, 50, 190, format_args!("",));
            // draw::write_string(&mut layer_2, 0, 190, format_args!("{}Joule: {} J{}\n{}Watt:  {} W", offset, joule, line1_end, offset, watt));
            draw::write_string(&mut layer_2, 180, 190, format_args!("Joule: {} J",  joule));
            draw::write_string(&mut layer_2, 180, 200, format_args!("Watt:  {} W",  watt));
            draw::write_string(&mut layer_2, 180, 210, format_args!("Click: {} J/C",  j_per_click));
            draw::write_string(&mut layer_2, 180, 220, format_args!("Time:  {} s",  time));
            draw::write_string(&mut layer_2, 180, 230, format_args!("CO2:   {} ppm",  emissions));
            // draw::write_string(&mut layer_2, 180, 230, format_args!("Demand:{} W",  demand));


            draw::write_string(&mut layer_2, 60, 190, format_args!("Powerplant",));
            draw::write_string(&mut layer_2, 330, 190, format_args!("Infrastructure",));

           

            if mode != 0 {
                mode_just_set = true;
            }

        }

        else if mode == 1 {
            if mode_just_set {
                if touch::touches(&mut i2c_3).unwrap().len() != 0 {
                    continue;
                }
                layer_1.clear();
                layer_2.clear();
                draw::draw_mode1(&mut layer_1, &mut layer_2);
                
                mode_just_set = false;
            }
          

            if touch::touches(&mut i2c_3).unwrap().len() == 0 {    
                clicker.reset_clicks();
            }

            else if touch::touches(&mut i2c_3).unwrap().len() == 1 {   
                for touch in &touch::touches(&mut i2c_3).unwrap() {  
                    let check_clicked = clicker.check_mode1_clicked((touch.x, touch.y));    
                    mode = check_clicked.1;
                    if check_clicked.0 {
                       clicker.update_watt();
                    }
                }
            }

            if mode != 1 {
                mode_just_set = true;
            }

            let solar_triple = clicker.get_solar();
            let wind_triple = clicker.get_wind();
            let gas_triple = clicker.get_gas();
            let coal_triple = clicker.get_coal();
            let nuclear_triple = clicker.get_nuclear();
            let hydro_triple = clicker.get_hydro();

	        draw::write_string(&mut layer_2, 120, 110, format_args!("Solar({}): {} W", solar_triple.1, solar_triple.2));
            draw::write_string(&mut layer_2, 240, 110, format_args!("Wind({}): {} W", wind_triple.1, wind_triple.2));
            draw::write_string(&mut layer_2, 360, 110, format_args!("Gas({}): {} W", gas_triple.1, gas_triple.2));

            draw::write_string(&mut layer_2, 120, 240, format_args!("Coal({}): ", coal_triple.1));
            draw::write_string(&mut layer_2, 120, 250, format_args!("{} W", coal_triple.2));
            draw::write_string(&mut layer_2, 240, 240, format_args!("Nuclear({}):", nuclear_triple.1));
            draw::write_string(&mut layer_2, 240, 250, format_args!("{} W", nuclear_triple.2));
            draw::write_string(&mut layer_2, 360, 240, format_args!("Hydro({}):", hydro_triple.1));
            draw::write_string(&mut layer_2, 360, 250, format_args!("{} W", hydro_triple.2));

            
            draw::write_string(&mut layer_2, 120, 120, format_args!("Cost: {} J", solar_triple.0));
            draw::write_string(&mut layer_2, 240, 120, format_args!("Cost: {} J", wind_triple.0));
            draw::write_string(&mut layer_2, 360, 120, format_args!("Cost: {} J", gas_triple.0));
            draw::write_string(&mut layer_2, 120, 260, format_args!("Cost: {} J", coal_triple.0));
            draw::write_string(&mut layer_2, 240, 260, format_args!("Cost: {} J", nuclear_triple.0));
            draw::write_string(&mut layer_2, 360, 260, format_args!("Cost: {} J", hydro_triple.0));




            // draw::write_string(&mut layer_2, 55, 125, format_args!("Solar"));
            // draw::write_string(&mut layer_2, 175, 125, format_args!("Wind"));
            // draw::write_string(&mut layer_2, 295, 125, format_args!("Coal"));
            
            // draw::write_string(&mut layer_2, 55, 135, format_args!("Solar"));
            // draw::write_string(&mut layer_2, 175, 135, format_args!("Wind"));
            // draw::write_string(&mut layer_2, 295, 135, format_args!("Coal"));


        }


         else if mode == 2 {
            if mode_just_set {
                if touch::touches(&mut i2c_3).unwrap().len() != 0 {
                    continue;
                }
                layer_1.clear();
                layer_2.clear();
                draw::draw_mode2(&mut layer_1, &mut layer_2);
                
                mode_just_set = false;
            }
          

            if touch::touches(&mut i2c_3).unwrap().len() == 0 {    
                clicker.reset_clicks();
            }

            else if touch::touches(&mut i2c_3).unwrap().len() == 1 {   
                for touch in &touch::touches(&mut i2c_3).unwrap() {  
                    let check_clicked = clicker.check_mode2_clicked((touch.x, touch.y));    
                    mode = check_clicked.1;
                    if check_clicked.0 {
                        clicker.update_joule_per_click();
                    }
                }
            }

            if mode != 2 {
                mode_just_set = true;
            }

            let battery_triple = clicker.get_battery();
            let smart_grid_triple = clicker.get_smart_grid();
            let hvac_triple = clicker.get_hvac();
            let hvdc_triple = clicker.get_hvdc();
            let tree_triple = clicker.get_tree();
            let special_triple = clicker.get_special();

	        draw::write_string(&mut layer_2, 120, 110, format_args!("AAA({}):{}J/C", battery_triple.1, battery_triple.2));
            draw::write_string(&mut layer_2, 240, 110, format_args!("Smart({}):{}J/C", smart_grid_triple.1, smart_grid_triple.2));
            draw::write_string(&mut layer_2, 360, 110, format_args!("HVAC({}):{}J/C", hvac_triple.1, hvac_triple.2));
            // draw::write_string(&mut layer_2, 120, 240, format_args!("HVDC({}):{}J/C", hvdc_triple.1, hvdc_triple.2));

            draw::write_string(&mut layer_2, 120, 240, format_args!("HVDC({}): ", hvdc_triple.1));
            draw::write_string(&mut layer_2, 120, 250, format_args!("{} J/C", hvdc_triple.2));
            draw::write_string(&mut layer_2, 240, 240, format_args!("Tree({}):", tree_triple.1));
            draw::write_string(&mut layer_2, 240, 250, format_args!("{} PPM/S", tree_triple.2));
            draw::write_string(&mut layer_2, 360, 240, format_args!("????({}):", special_triple.1));
            draw::write_string(&mut layer_2, 360, 250, format_args!("{} J/C", special_triple.2));

            
            draw::write_string(&mut layer_2, 120, 120, format_args!("Cost: {} J", battery_triple.0));
            draw::write_string(&mut layer_2, 240, 120, format_args!("Cost: {} J", smart_grid_triple.0));
            draw::write_string(&mut layer_2, 360, 120, format_args!("Cost: {} J", hvac_triple.0));
            
            draw::write_string(&mut layer_2, 120, 260, format_args!("Cost: {} J", hvdc_triple.0));
            draw::write_string(&mut layer_2, 240, 260, format_args!("Cost: {} J", tree_triple.0));
            draw::write_string(&mut layer_2, 360, 260, format_args!("Cost: {} J", special_triple.0));




            // draw::write_string(&mut layer_2, 55, 125, format_args!("Solar"));
            // draw::write_string(&mut layer_2, 175, 125, format_args!("Wind"));
            // draw::write_string(&mut layer_2, 295, 125, format_args!("Coal"));
            
            // draw::write_string(&mut layer_2, 55, 135, format_args!("Solar"));
            // draw::write_string(&mut layer_2, 175, 135, format_args!("Wind"));
            // draw::write_string(&mut layer_2, 295, 135, format_args!("Coal"));


        }

        else if mode == 3 {
            if mode_just_set {
                layer_1.clear();
                layer_2.clear();

                draw::write_string(&mut layer_2, 20, 100, format_args!("Climate change has defeated humanity!"));
                draw::write_string(&mut layer_2, 20, 110, format_args!("However, unlike earth, you can respawn in this game!"));
                draw::write_string(&mut layer_2, 20, 120, format_args!("Touch the screen to start again."));
                mode_just_set = false;
                clicker = clicker::Clicker::new(powerplant::Powerplant::new(), infrastructure::Infrastructure::new(), carbon_dioxide::Carbondioxide::new());
                emissions = 0;
            }
            if touch::touches(&mut i2c_3).unwrap().len() == 1 {
                mode = 0;
                mode_just_set = true;
            }
            
        }
            // clicker_color = sky_blue;


          else if mode == 4 {
            //draw::write_string(&mut layer_2, 20, 120, format_args!("Touch the screen to start again."));

            if mode_just_set {
                layer_1.clear();
                layer_2.clear();
                draw::write_string(&mut layer_2, 80, 200, format_args!("Touch with two fingers to continue."));
                let random = rng.poll_and_get().expect("Failed to generate random number") % 6;
                mode_just_set = false;
                if random == 0 {
                    clicker.get_powerplant().reset_solar();
                    bmp_reader::draw_image(&mut layer_1, "scheuer", 0, 0);
                    draw::write_string(&mut layer_2, 20, 120, format_args!("A sun exploded amd your Solar Panels have been destroyed."));
                    draw::write_string(&mut layer_2, 20, 150, format_args!("The sun will respawn momentarily."));
                }
                if random == 1 {
                    clicker.get_powerplant().reset_wind();
                    bmp_reader::draw_image(&mut layer_1, "scheuer", 0, 0);
                    draw::write_string(&mut layer_2, 20, 120, format_args!("A blue whale landed on your wind farm."));
                    draw::write_string(&mut layer_2, 20, 150, format_args!("Obviously they have all been destroyed."));
                }
                if random == 2 {
                    clicker.get_carbon_dioxide().remove_trees(50);
                    bmp_reader::draw_image(&mut layer_1, "scheuer", 0, 0);
                    draw::write_string(&mut layer_2, 20, 120, format_args!("Your trees have been slashed 'n' burned."));
                    draw::write_string(&mut layer_2, 20, 150, format_args!("You lost up to 50 trees."));
                } 
                if random == 3 {
                    clicker.get_powerplant().add_bonus_emissions(50);
                    bmp_reader::draw_image(&mut layer_1, "scheuer", 0, 0);
                    draw::write_string(&mut layer_2, 20, 120, format_args!("Andreas Scheuer wants to VROOM VROOM."));
                    draw::write_string(&mut layer_2, 20, 150, format_args!("Your ppm/s increases by 50."));
                } 
                if random == 4 {
                    clicker.get_powerplant().reset_gas();
                    clicker.get_powerplant().reset_coal();
                    clicker.get_powerplant().reset_nuclear();
                    bmp_reader::draw_image(&mut layer_1, "scheuer", 0, 0);
                    draw::write_string(&mut layer_2, 20, 120, format_args!("Purring and threatening dog ate your Powerpflanzen..."));
                    draw::write_string(&mut layer_2, 20, 150, format_args!("...except renewables."));
                } 
                if random == 5 {
                    clicker.get_powerplant().reset_nuclear();
                    bmp_reader::draw_image(&mut layer_1, "scheuer", 0, 0);
                    draw::write_string(&mut layer_2, 20, 120, format_args!("Greenpeace flies a Superman drone "));
                    draw::write_string(&mut layer_2, 20, 130, format_args!("into a nuclear power plant."));
                    draw::write_string(&mut layer_2, 20, 150, format_args!("All your nuclear power plants have been obliterated."));
                }
                if random > 5 {
                    mode = 0;
                    mode_just_set = true;
                } 
                clicker.update_watt();
                bmp_reader::draw_image(&mut layer_1, "x", 440, 0);
            }

            if touch::touches(&mut i2c_3).unwrap().len() == 0 {    
                clicker.reset_clicks();
            }

            else if touch::touches(&mut i2c_3).unwrap().len() == 1 {   
                for touch in &touch::touches(&mut i2c_3).unwrap() {  
                    if touch.x > 438 && touch.y < 230 {
                        mode = 0;
                        mode_just_set = true;
                    }
                }
            }    
        }

      
  
        // let mut text1 = layer_1.text_writer();
       
        // let mut text2 = layer_2.text_writer();


        // text2.x_pos = 0;
        //text2.x_pos = 0;
        // text2.y_pos = 200;
        
        // text2.write_fmt(format_args!("{}Joule: {} J", offset, joule));

        // text2.write_fmt(format_args!("\n{}Watt:  {} W", offset, watt));
     

        

        // for x in 0..480 {
        //     for y in 0..272 {
        //         if (x == 160 || x == 320) && (y >= 61 && y <= 211) {
        //             layer_1.print_point_color_at(x, y, black);
                                    
        //         }
        //         if (y == 61 || y == 211) && (x >= 160 && x <= 320) {
        //             layer_1.print_point_color_at(x, y, black);
                    
        //         }

        //         if (x > 160 && x < 320) && (y > 61 && y < 211) {
        //             layer_1.print_point_color_at(x, y, clicker_color);
                                    
        //         }

        //     }
        // }


        

    }
}




#[global_allocator]
static ALLOCATOR: CortexMHeap = CortexMHeap::empty();

#[exception]
fn SysTick() {
    system_clock::tick();
}

// define what happens in an Out Of Memory (OOM) condition
#[alloc_error_handler]
fn rust_oom(_: AllocLayout) -> ! {
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use core::fmt::Write;
    use cortex_m::asm;
    use cortex_m_semihosting::hio;

    if let Ok(mut hstdout) = hio::hstdout() {
        let _ = writeln!(hstdout, "{}", info);
    }

    // OK to fire a breakpoint here because we know the microcontroller is connected to a debugger
    asm::bkpt();

    loop {}
}
