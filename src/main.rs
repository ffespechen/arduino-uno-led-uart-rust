/*
* Leer desde el serial y encender/apagar el led correspondiende
*/

#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;


#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    // LEDs
    let mut pin_rojo = pins.d8.into_output_high();
    let mut pin_amarillo = pins.d9.into_output_high();
    let mut pin_verde = pins.d10.into_output_high();
    let mut pin_azul = pins.d11.into_output_high();

    let mut pin_blanco = pins.d7.into_output();


    ufmt::uwriteln!(&mut serial, "Comandando leds desde serial").unwrap();
    ufmt::uwriteln!(&mut serial, "R -> rojo \nA-> amarillo \nV -> verde \nZ -> azul ").unwrap();

    loop {
        let b: u8 = nb::block!(serial.read()).unwrap();

        match b {
            b'R' => pin_rojo.toggle(),
            b'A' => pin_amarillo.toggle(),
            b'V' => pin_verde.toggle(),
            b'Z' => pin_azul.toggle(),
            _ => pin_blanco.toggle()           
        }
    }    

}
