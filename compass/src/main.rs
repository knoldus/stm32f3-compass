#![no_main]
#![no_std]

#[allow(unused_imports)]
use stm32_compass::{entry, init, iprintln, switch_hal::OutputSwitch, Direction};
use stm32f3_discovery::stm32f3xx_hal::prelude::*;
/// This program is going to print the (x,y) axis values on itm terminal and will blink the led
/// based on the (x,y)->Magnetic Field Direction.
/// This program will use the discovery board as a Compass.
///
/// #Return
/// Program is using [no_std] & [no_main] therefore it will neither end nor return anything.
#[entry]
fn main() -> ! {
    let (leds, mut lsm303agr, mut delay, mut itm) = init();
    let mut stm_leds = leds.into_array();

    loop {
        // Reading the mag register value using mag_status().
        let x_y_axis = lsm303agr.mag_data().unwrap();
        let direction = match (x_y_axis.x > 0, x_y_axis.y > 0) {
            (true, true) => Direction::Southeast,
            (false, true) => Direction::Northeast,
            (true, false) => Direction::Southwest,
            (false, false) => Direction::Northwest,
        };

        stm_leds.iter_mut().for_each(|leds| leds.off().unwrap());
        stm_leds[direction as usize].on().unwrap();
        iprintln!(&mut itm.stim[0], "x = {} y = {}", x_y_axis.x, x_y_axis.y);

        delay.delay_ms(1_000_u16);
    }
}