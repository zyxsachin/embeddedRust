#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![warn(clippy::all)]

use core::fmt::Write;
use alloc_cortex_m::CortexMHeap;
use core::alloc::Layout as AllocLayout;
use core::panic::PanicInfo;
use cortex_m_rt::{entry, exception};
use stm32f7::stm32f7x6::{CorePeripherals, Peripherals};
use stm32f7_discovery::{
    gpio::{GpioPort, OutputPin},
    init,
    system_clock::{self, Hz},  touch, lcd::Color,
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

    // configure the systick timer 20Hz (20 ticks per second)
    init::init_systick(Hz(20), &mut systick, &rcc);
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
    let grey = Color{red: 127,green: 127 ,blue: 127,alpha: 127};

    let sky_blue = Color{red: 51, green: 204, blue: 255, alpha:255};
    layer_1.clear();
    layer_2.clear();
    lcd.set_background_color(sky_blue);


    let arr = [24,25,49,50,74,75,99,100,124,125,149,150,174,175,199,200,224,225,249,250,274,275];
    //let arr = [200,210,220];
    //let arr = [10,20,30];
    let arr2 = [24,25,49,50,74,75,99,100,124,125,149,150,174,175,199,200,224,225,249,250];

    // turn led on
    pins.led.set(true);
    let mut count = 0;
    let mut last_led_toggle = system_clock::ticks();
    loop {
        let ticks = system_clock::ticks();
        // every 0.5 seconds (we have 20 ticks per second)
        if ticks - last_led_toggle >= 10 {
            pins.led.toggle();
            last_led_toggle = ticks;
            

        }
        let mut text1 = layer_1.text_writer();
  
        //let mut text2 = layer_2.text_writer();

        text1.x_pos = 165;
        text1.y_pos = 200;

        

        // text1.write_str("&lcd.layer_1()n\n\n\n\n");
        // text1.write_str("\n\n\n\n\n");

        // text1.write_str("\n\n\n\n\n");
        // text1.write_str("\n\n\n\n\n");

        // text1.write_str("\n\n\n\n\n");
        // text1.write_str("\n\n\n\n\n");
        // text1.write_str("\n\n\n\n");

        text1.write_str("Count:");
     



        

        for x in 0..480 {
            for y in 0..272 {
                if (x == 160 || x == 320) && (y >= 61 && y < 211) {
                    layer_1.print_point_color_at(x, y, black);
                                    
                }
                if (y == 61 || y == 211) && (x >= 160 && x < 320) {
                    layer_1.print_point_color_at(x, y, black);
                    
                }
            }
        }


        // for c in arr.iter() {
        //     //let i1 = 124 + 5 * c;
        //     //let i2 = 356 - 5 * c;
        //     //let j1 = 10 + 5 * c;
        //     //let j2 = 262 - 5 * c;
        //     //for i in i1..i2 {
        //     for i in 0..272 {
        //         layer_2.print_point_color_at(*c, i, green);
        //         //for j in j1..j2 {
        //         //    layer_1.print_point_color_at(i, j, blue);
        //         //}
        //     }
        // }
        // for c in arr2.iter() {
        //     for i in 0..275 {
        //         layer_1.print_point_color_at(i, *c, black);
        //     }
        // }
        // for c in 53..72 {
        //     for i in 78..172 {
        //         layer_2.print_point_color_at(c, i, grey);
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
