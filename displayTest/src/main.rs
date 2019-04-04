#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![warn(clippy::all)]
#![feature(alloc)]

#[macro_use]
extern crate alloc;

mod clicker;
mod draw;

use alloc::sync::Arc;
use alloc::vec::Vec;
use stm32f7_discovery::future_mutex;



//use alloc::fmt as fmt;
use core::fmt::Write;

use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout as AllocLayout;
use core::panic::PanicInfo;
use cortex_m_rt::{entry, exception};
use stm32f7::stm32f7x6::{CorePeripherals, Peripherals};
use stm32f7_discovery::{
    gpio::{GpioPort, OutputPin},
    init,
    system_clock::{self, Hz},  touch, lcd::Color, i2c
};



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
    let mut sai_2 = peripherals.SAI2;
    let mut rng = peripherals.RNG;
    let mut sdmmc = peripherals.SDMMC1;
    let mut syscfg = peripherals.SYSCFG;
    let mut ethernet_mac = peripherals.ETHERNET_MAC;
    let mut ethernet_dma = peripherals.ETHERNET_DMA;

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


    let mut clicker = clicker::Clicker::new();

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
    
    let bg_color = Color{red: 255,green: 0 ,blue: 0,alpha: 255};
    let blue = Color{red: 0,green: 0 ,blue: 255,alpha: 255};
    let green = Color{red: 0,green: 255 ,blue: 0,alpha: 255};
    let black = Color{red: 0,green: 0 ,blue: 0,alpha: 255};
    let white = Color{red: 255,green: 255 ,blue: 255,alpha: 255};
    let grey = Color{red: 127,green: 127 ,blue: 127,alpha: 127};
    let yellow = Color{red: 255, green: 255, blue: 0, alpha: 255};
    let red = Color{red: 255, green: 0, blue: 0, alpha: 255};

    let sky_blue = Color{red: 51, green: 204, blue: 255, alpha:255};
    layer_1.clear();
    layer_2.clear();
    lcd.set_background_color(sky_blue);


    // turn led on
    pins.led.set(true);
    let mut last_led_toggle = system_clock::ticks();

    
    let mut clicked_ticks = 0;
    let mut clicker_color = sky_blue;
      
        
    let max_x = 480;
    let max_y = 272;
    let centre_x = max_x / 2;
    let centre_y = max_y / 2;
    let radius = 50;

    let mut mode = 0;
    let mut mode_just_set = false;

    let mut circle_reset = false;

    draw::draw_circle(&mut layer_1, centre_x, centre_y, radius, yellow);
    draw::draw_rectangle(&mut layer_1, 50, 80, 100, 100, black);
    draw::draw_rectangle(&mut layer_1, 330, 80, 100, 100, black);
    



    let mut old_tick = system_clock::ticks();;

    loop {
        let ticks = system_clock::ticks();

        if mode == 0 {
            if circle_reset {
                draw::color_circle(&mut layer_1, centre_x, centre_y, radius, sky_blue);
                circle_reset = false;
            }
            if mode_just_set {
                draw::draw_circle(&mut layer_1, centre_x, centre_y, radius, yellow);
                mode_just_set = false;
            }
            if ticks - old_tick >= 100 {
                old_tick = ticks;
                clicker.increment();

            }

            if touch::touches(&mut i2c_3).unwrap().len() == 0 {
                clicker.reset_clicks();
                
            }
            else if touch::touches(&mut i2c_3).unwrap().len() == 1 {    
                for touch in &touch::touches(&mut i2c_3).unwrap() {  
                    let check_clicked = clicker.check_mode0_clicked((touch.x, touch.y));    
                     
                    if check_clicked == 0 {
                        clicker.energy_tick();
                        draw::color_circle(&mut layer_1, centre_x, centre_y, radius, yellow);
                        circle_reset = true;
                    }
                }
            }
 
            let joule = clicker.get_joule();
            let watt = clicker.get_watt();
            let offset = "                         ";
            draw::write_string(&mut layer_2, 0, 190, format_args!("{}Joule: {} J\n{}Watt:  {} W", offset, joule, offset, watt));


        }
            clicker_color = sky_blue;


        // else if mode == 1 {

        // }
            
    

      
  
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
